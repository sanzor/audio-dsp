use std::collections::HashMap;
use std::sync::Arc;

use domain::db::{
    db_transform::{DbTransform, TransformId},
    db_transform_draft::{DbTransformDraft, TransformDraftId},
};
use domain::domain_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::ticket_worker::processor::{
    build_job::check_transform_source,
    build_job_config::BuildJobConfig,
    transform_metadata::{PortMetadataJson, TransformMetadataJson},
};

use super::{
    data_provider::transform_drafts_data_provider::TransformDraftsDataProvider,
    transform_drafts_provider::TransformDraftsProvider,
    validator::{
        graph_definition::GraphDefinition, transnform_info::TransformInfo, validator::{Validator, ValidatorInput},
    },
};

/// Guards the kind-specific save/publish endpoints against being called on
/// the wrong kind of draft (e.g. `save_primitive_draft` on a composite) —
/// a client bug that should surface as a clear 400, not a confusing
/// downstream failure (a composite draft has no `wasm_bytecode` to publish;
/// a primitive draft has no graph to validate).
fn require_kind(draft: &DbTransformDraft, expected: &str) -> Result<(), ServiceError> {
    if draft.kind != expected {
        return Err(ServiceError::Validation(format!(
            "transform {} is a '{}', not a '{expected}'",
            draft.transform_id, draft.kind
        )));
    }
    Ok(())
}

pub struct TransformDraftsProviderService {
    data: Arc<dyn TransformDraftsDataProvider>,
    build_job_config: BuildJobConfig,
}

impl TransformDraftsProviderService {
    pub fn new(data: Arc<dyn TransformDraftsDataProvider>, build_job_config: BuildJobConfig) -> Self {
        Self { data, build_job_config }
    }

    /// Fetches already-published transforms by id (silently omitting any
    /// that don't exist or were never published) and reduces each to what
    /// `Validator` needs: its kind and its derived ports. An unpublished
    /// draft's `transform` row has `metadata = NULL`, which is dropped here
    /// — exactly the "doesn't exist or has never been published" case
    /// `Validator` already treats as absence from the map.
    async fn fetch_leaf_defs(&self, ids: &[TransformId]) -> Result<HashMap<TransformId, TransformInfo>, ServiceError> {
        if ids.is_empty() {
            return Ok(HashMap::new());
        }

        // `get_published_transforms` never errors NotFound if an id is
        // missing — a referenced transform not existing (or never
        // published) is a normal, expected `Validator` failure case, not a
        // hard error here.
        let transforms = self.data.get_published_transforms(ids).await.map_err(ServiceError::from)?;

        Ok(transforms
            .into_iter()
            .filter_map(|t| {
                let meta: TransformMetadataJson = serde_json::from_str(t.metadata.as_deref()?).ok()?;
                Some((t.transform_id, TransformInfo { kind: t.kind, ports: meta.ports }))
            })
            .collect())
    }
}

#[async_trait::async_trait]
impl TransformDraftsProvider for TransformDraftsProviderService {
    async fn get_transform_draft(&self, id: TransformDraftId) -> Result<DbTransformDraft, ServiceError> {
        self.data.get_transform_draft(id).await.map_err(ServiceError::from)
    }

    async fn get_transform_drafts(&self, ids: &[TransformDraftId]) -> Result<Vec<DbTransformDraft>, ServiceError> {
        let drafts = self.data.get_transform_drafts(ids).await?;
        let found: std::collections::HashSet<TransformDraftId> = drafts.iter().map(|d| d.transform_id).collect();
        let missing: Vec<TransformDraftId> = ids.iter().copied().filter(|id| !found.contains(id)).collect();
        if missing.is_empty() {
            Ok(drafts)
        } else {
            Err(ServiceError::NotFound)
        }
    }

    async fn get_transform_draft_owner(&self, id: TransformDraftId) -> Result<UserId, ServiceError> {
        self.data.get_transform_owner(id).await.map_err(ServiceError::from)
    }

    async fn create_transform_draft(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
        kind: String,
        owner_user_id: UserId,
    ) -> Result<DbTransformDraft, ServiceError> {
        self.data
            .insert_transform_draft(name, description, icon, kind, owner_user_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn save_primitive_draft(
        &self,
        id: TransformDraftId,
        source_code: String,
    ) -> Result<DbTransformDraft, ServiceError> {
        require_kind(&self.data.get_transform_draft(id).await?, "primitive")?;
        self.data.save_primitive_draft(id, source_code).await.map_err(ServiceError::from)
    }

    async fn save_composite_draft(&self, id: TransformDraftId, graph_json: String) -> Result<DbTransformDraft, ServiceError> {
        require_kind(&self.data.get_transform_draft(id).await?, "composite")?;
        self.data.save_composite_draft(id, graph_json).await.map_err(ServiceError::from)
    }

    async fn check_source(&self, source_code: String) -> Result<(), ServiceError> {
        check_transform_source(&self.build_job_config, &source_code)
            .await
            .map_err(ServiceError::Validation)
    }

    /// See `TransformDraftsProvider::validate_graph_draft`. `id` scopes
    /// ownership only — the graph validated is whatever `graph_json` the
    /// caller passes, not necessarily what's currently saved, so the
    /// Creator can validate live in-progress edits before saving.
    async fn validate_graph_draft(&self, _id: TransformDraftId, graph_json: String) -> Result<Vec<PortMetadataJson>, ServiceError> {
        let referenced_ids = serde_json::from_str::<GraphDefinition>(&graph_json)
            .map(|g| g.referenced_transform_ids())
            .unwrap_or_default();

        let leaf_defs = self.fetch_leaf_defs(&referenced_ids).await?;

        Validator::new()
            .validate(ValidatorInput { metadata_json: graph_json, leaf_defs })
            .map_err(ServiceError::Validation)
    }

    async fn publish_primitive(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError> {
        let draft = self.data.get_transform_draft(id).await?;
        require_kind(&draft, "primitive")?;

        let Some(wasm_bytecode) = draft.wasm_bytecode else {
            return Err(ServiceError::Validation(
                "nothing has been saved with a successful build yet".to_string(),
            ));
        };

        // Publish requires the saved binary and saved source to correspond
        // — a source-only save leaves a previously-attached binary in place
        // without updating it, so a later source-only save can move
        // source_code forward while wasm_bytecode/wasm_source_code stay put.
        // See agents/decisions/0002-transform-draft-lifecycle-decisions.md.
        if draft.wasm_source_code.as_deref() != draft.source_code.as_deref() {
            return Err(ServiceError::Validation(
                "saved binary no longer corresponds to the saved source; recompile and re-attach before publishing".to_string(),
            ));
        }

        self.data
            .publish_compiled_transform(
                id,
                wasm_bytecode,
                draft.source_code.unwrap_or_default(),
                draft.name.unwrap_or_default(),
                draft.description,
                draft.metadata.unwrap_or_default(),
            )
            .await
            .map_err(ServiceError::from)
    }

    /// Re-validates the currently-saved graph (cheap — no ticket, no
    /// binary) and publishes the derived ports alongside it, wrapped into
    /// the same `{name, description, ports, params, graph}` envelope any
    /// transform's `metadata` carries — this is what lets this composite
    /// later be referenced as a leaf inside another composite's graph.
    /// `wasm_bytecode` is left untouched (`NULL`, always, for a composite —
    /// there's no `resolve` step yet to give it one).
    async fn publish_composite(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError> {
        let draft = self.data.get_transform_draft(id).await?;
        require_kind(&draft, "composite")?;

        let graph_json = draft.metadata.ok_or_else(|| {
            ServiceError::Validation("nothing has been saved yet".to_string())
        })?;

        let referenced_ids = serde_json::from_str::<GraphDefinition>(&graph_json)
            .map(|g| g.referenced_transform_ids())
            .unwrap_or_default();
        let leaf_defs = self.fetch_leaf_defs(&referenced_ids).await?;
        let ports = Validator::new()
            .validate(ValidatorInput { metadata_json: graph_json.clone(), leaf_defs })
            .map_err(ServiceError::Validation)?;

        let graph: GraphDefinition = serde_json::from_str(&graph_json)
            .map_err(|e| ServiceError::Internal(format!("graph validated but failed to re-parse: {e}")))?;
        let metadata = TransformMetadataJson {
            name: draft.name.clone().unwrap_or_default(),
            description: draft.description.clone(),
            ports,
            params: Vec::new(),
            graph: Some(graph),
        };
        let metadata_json = serde_json::to_string(&metadata)
            .map_err(|e| ServiceError::Internal(format!("failed to serialize composite metadata: {e}")))?;

        self.data
            .publish_composite_transform(id, draft.name.unwrap_or_default(), draft.description, metadata_json)
            .await
            .map_err(ServiceError::from)
    }

    async fn delete_transform_draft(&self, id: TransformDraftId) -> Result<(), ServiceError> {
        self.data.delete_transform_draft(id).await.map_err(ServiceError::from)
    }
}

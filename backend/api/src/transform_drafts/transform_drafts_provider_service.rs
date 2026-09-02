use std::collections::HashMap;
use std::sync::Arc;

use crate::ticket_worker::processor::{
    build_job::check_transform_source,
    build_job_config::BuildJobConfig,
    transform_metadata::{PortMetadataJson, TransformMetadataJson},
};
use crate::{
    domain::service_error::ServiceError,
    transform_drafts::dto::requests::{SaveCompositeParams, SaveDraftParams, SavePrimitiveParams},
};
use base64::{prelude::BASE64_STANDARD, Engine};
use domain::db::{
    db_transform::{DbTransform, TransformId},
    db_transform_draft::{DbTransformDraft, TransformDraftId},
};
use domain::domain_user::UserId;

use super::{
    data_provider::transform_drafts_data_provider::{
        CompiledPrimitiveDraft, TransformDraftsDataProvider,
    },
    graph_validator::{
        graph_definition::GraphDefinition,
        transform_info::TransformInfo,
        validator::{Validator, ValidatorInput},
    },
    transform_drafts_provider::TransformDraftsProvider,
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
    metadata_fuel_limit: u64,
}

impl TransformDraftsProviderService {
    pub fn new(
        data: Arc<dyn TransformDraftsDataProvider>,
        build_job_config: BuildJobConfig,
        metadata_fuel_limit: u64,
    ) -> Self {
        Self {
            data,
            build_job_config,
            metadata_fuel_limit,
        }
    }

    async fn fetch_leaf_defs(
        &self,
        ids: &[TransformId],
    ) -> Result<HashMap<TransformId, TransformInfo>, ServiceError> {
        if ids.is_empty() {
            return Ok(HashMap::new());
        }

        // `get_published_transforms` never errors NotFound if an id is
        // missing — a referenced transform not existing (or never
        // published) is a normal, expected `Validator` failure case, not a
        // hard error here.
        let transforms = self
            .data
            .get_published_transforms(ids)
            .await
            .map_err(ServiceError::from)?;

        Ok(transforms
            .into_iter()
            .filter_map(|t| {
                let meta: TransformMetadataJson =
                    serde_json::from_str(t.metadata.as_deref()?).ok()?;
                Some((
                    t.transform_id,
                    TransformInfo {
                        kind: t.kind,
                        ports: meta.ports,
                    },
                ))
            })
            .collect())
    }
}

#[async_trait::async_trait]
impl TransformDraftsProvider for TransformDraftsProviderService {
    async fn get_transform_draft(
        &self,
        id: TransformDraftId,
    ) -> Result<DbTransformDraft, ServiceError> {
        self.data
            .get_transform_draft(id)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_transform_drafts(
        &self,
        ids: &[TransformDraftId],
    ) -> Result<Vec<DbTransformDraft>, ServiceError> {
        let drafts = self.data.get_transform_drafts(ids).await?;
        let found: std::collections::HashSet<TransformDraftId> =
            drafts.iter().map(|d| d.transform_id).collect();
        let missing: Vec<TransformDraftId> = ids
            .iter()
            .copied()
            .filter(|id| !found.contains(id))
            .collect();
        if missing.is_empty() {
            Ok(drafts)
        } else {
            Err(ServiceError::NotFound)
        }
    }

    async fn get_transform_draft_owner(
        &self,
        id: TransformDraftId,
    ) -> Result<UserId, ServiceError> {
        self.data
            .get_transform_owner(id)
            .await
            .map_err(ServiceError::from)
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
    async fn save_draft(
        &self,
        id: TransformDraftId,
        save_params: SaveDraftParams,
    ) -> Result<DbTransformDraft, ServiceError> {
        match save_params {
            SaveDraftParams::Primitive(params) => self.save_primitive_draft(id, params).await,
            SaveDraftParams::Composite(params) => self.save_composite_draft(id, params).await,
        }
    }
    async fn publish(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError> {
        let draft = self.data.get_transform_draft(id).await?;
        match draft.kind.as_str() {
            "primitive" => self.publish_primitive(id).await,
            "composite" => self.publish_composite(id).await,
            kind => Err(ServiceError::Validation(format!(
                "transform {id} has unsupported kind '{kind}'"
            ))),
        }
    }

    async fn check_source(&self, source_code: String) -> Result<(), ServiceError> {
        check_transform_source(&self.build_job_config, &source_code)
            .await
            .map_err(ServiceError::Validation)
    }

    async fn validate_graph_draft(
        &self,
        _id: TransformDraftId,
        graph_json: String,
    ) -> Result<Vec<PortMetadataJson>, ServiceError> {
        let referenced_ids = serde_json::from_str::<GraphDefinition>(&graph_json)
            .map(|g| g.referenced_transform_ids())
            .unwrap_or_default();
        let leaf_defs = self.fetch_leaf_defs(&referenced_ids).await?;

        Validator::new()
            .validate(ValidatorInput {
                metadata_json: graph_json,
                leaf_defs,
            })
            .map_err(ServiceError::Validation)
    }

    async fn delete_transform_draft(&self, id: TransformDraftId) -> Result<(), ServiceError> {
        self.data
            .delete_transform_draft(id)
            .await
            .map_err(ServiceError::from)
    }
}

impl TransformDraftsProviderService {
    async fn save_primitive_draft(
        &self,
        id: TransformDraftId,
        params: SavePrimitiveParams,
    ) -> Result<DbTransformDraft, ServiceError> {
        require_kind(&self.data.get_transform_draft(id).await?, "primitive")?;
        let wasm_bytecode = params
            .wasm_base64
            .map(|encoded| BASE64_STANDARD.decode(encoded))
            .transpose()
            .map_err(|_| {
                ServiceError::Validation("wasm_base64 must be valid base64".to_string())
            })?;

        self.data
            .save_primitive_draft(id, params.source_code, compiled)
            .await
            .map_err(ServiceError::from)
    }
    async fn process_bytecode(
        &self,
        wasm_bytecode: Option<Vec<u8>>,
    ) -> Result<Option<CompiledPrimitiveDraft>, ServiceError> {
        let Some(wasm_bytecode) = wasm_bytecode else {
            return Ok(None);
        };

        if wasm_bytecode.len() as u64 > self.build_job_config.max_wasm_bytes {
            return Err(ServiceError::Validation(format!(
                "compiled wasm exceeds the {} byte limit",
                self.build_job_config.max_wasm_bytes
            )));
        }

        let metadata = crate::ticket_worker::processor::metadata_introspector::introspect_metadata(
            &wasm_bytecode,
            self.metadata_fuel_limit,
        )
        .map_err(ServiceError::Validation)?;

        let metadata_json = serde_json::to_string(&metadata).map_err(|e| {
            ServiceError::Internal(format!("failed to serialize compiled metadata: {e}"))
        })?;

        Ok(Some(CompiledPrimitiveDraft {
            wasm_bytecode,
            name: metadata.name,
            description: metadata.description,
            metadata: metadata_json,
        }))
    }
    async fn save_composite_draft(
        &self,
        id: TransformDraftId,
        params: SaveCompositeParams,
    ) -> Result<DbTransformDraft, ServiceError> {
        require_kind(&self.data.get_transform_draft(id).await?, "composite")?;
        let graph_json = serde_json::to_string(&params.graph_definition).map_err(|e| {
            ServiceError::Validation(format!("graph_definition must be valid JSON: {e}"))
        })?;
        self.data
            .save_composite_draft(id, graph_json)
            .await
            .map_err(ServiceError::from)
    }
    async fn publish_composite(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError> {
        let draft = self.data.get_transform_draft(id).await?;
        require_kind(&draft, "composite")?;

        let graph_json = draft
            .metadata
            .ok_or_else(|| ServiceError::Validation("nothing has been saved yet".to_string()))?;

        let referenced_ids = serde_json::from_str::<GraphDefinition>(&graph_json)
            .map(|g| g.referenced_transform_ids())
            .unwrap_or_default();
        let leaf_defs = self.fetch_leaf_defs(&referenced_ids).await?;
        let ports = Validator::new()
            .validate(ValidatorInput {
                metadata_json: graph_json.clone(),
                leaf_defs,
            })
            .map_err(ServiceError::Validation)?;

        let graph: GraphDefinition = serde_json::from_str(&graph_json).map_err(|e| {
            ServiceError::Internal(format!("graph validated but failed to re-parse: {e}"))
        })?;
        let metadata = TransformMetadataJson {
            name: draft.name.clone().unwrap_or_default(),
            description: draft.description.clone(),
            ports,
            params: Vec::new(),
            graph: Some(graph),
        };
        let metadata_json = serde_json::to_string(&metadata).map_err(|e| {
            ServiceError::Internal(format!("failed to serialize composite metadata: {e}"))
        })?;

        self.data
            .publish_composite_transform(
                id,
                draft.name.unwrap_or_default(),
                draft.description,
                metadata_json,
            )
            .await
            .map_err(ServiceError::from)
    }
    async fn publish_primitive(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError> {
        let draft = self.data.get_transform_draft(id).await?;
        require_kind(&draft, "primitive")?;

        let Some(wasm_bytecode) = draft.wasm_bytecode else {
            return Err(ServiceError::Validation(
                "nothing has been saved with a successful build yet".to_string(),
            ));
        };

        
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
}

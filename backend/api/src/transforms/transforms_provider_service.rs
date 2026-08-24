use std::collections::HashMap;
use std::sync::Arc;

use domain::db::{
    db_transform::{DbTransform, TransformId},
    db_transform_draft::{DbTransformDraft, TransformDraftId},
    ticket::db_resource::ResourceId,
    WorkspaceId,
};
use domain::domain_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::ticket_worker::processor::{
    build_job::check_transform_source,
    build_job_config::BuildJobConfig,
    transform_metadata::{PortMetadataJson, TransformMetadataJson},
};

use super::{
    data_provider::transforms_data_provider::TransformsDataProvider,
    transforms_provider::TransformsProvider,
    validator::{
        graph_definition::GraphDefinition, transnform_info::TransformInfo, validator::{Validator, ValidatorInput},
    },
};

/// Mirror of `transforms_data_provider_service`'s `metadata_from_words` —
/// `DbTransformDraft.metadata`/`DbTransform.metadata` are `Vec<u32>` (one
/// word per UTF-8 byte of the underlying JSON string; see that file's doc
/// comment on `metadata_to_words` for why), but `publish_compiled_transform`
/// takes the JSON back as a plain `String` to hand to the data layer. Small,
/// deliberate duplication rather than threading a shared helper through a
/// crate boundary for one conversion.
fn words_to_metadata_string(words: &[u32]) -> String {
    let bytes: Vec<u8> = words.iter().map(|&w| w as u8).collect();
    String::from_utf8(bytes).unwrap_or_default()
}

/// The bucket-2 "publishable" predicate `is_draft_publishable` reports and
/// `publish_transform` itself gates on (there re-checked field-by-field for
/// distinct error messages, rather than calling this directly).
fn is_bucket2_publishable(draft: &DbTransformDraft) -> bool {
    draft.wasm_bytecode.is_some() && draft.wasm_source_code.as_deref() == Some(draft.source_code.as_str())
}

pub struct TransformsProviderService {
    data: Arc<dyn TransformsDataProvider>,
    build_job_config: BuildJobConfig,
}

impl TransformsProviderService {
    pub fn new(data: Arc<dyn TransformsDataProvider>, build_job_config: BuildJobConfig) -> Self {
        Self { data, build_job_config }
    }

    /// Fetches already-published transforms by id (silently omitting any
    /// that don't exist or were never published) and reduces each to what
    /// `Validator` needs: its kind and its derived ports. An unpublished
    /// draft's `transform` row has empty `metadata`, which fails to parse
    /// and is dropped here — exactly the "doesn't exist or has never been
    /// published" case `Validator` already treats as absence from the map.
    async fn fetch_leaf_defs(&self, ids: &[TransformId]) -> Result<HashMap<TransformId, TransformInfo>, ServiceError> {
        if ids.is_empty() {
            return Ok(HashMap::new());
        }

        // The data provider directly, not `self.get_transforms` — that
        // trait method errors NotFound if any id is missing, but a
        // referenced transform not existing (or never published) is a
        // normal, expected `Validator` failure case, not a hard error here.
        let transforms = self.data.get_transforms(ids).await.map_err(ServiceError::from)?;

        Ok(transforms
            .into_iter()
            .filter_map(|t| {
                let meta: TransformMetadataJson = serde_json::from_str(&words_to_metadata_string(&t.metadata)).ok()?;
                Some((t.transform_id, TransformInfo { kind: t.kind, ports: meta.ports }))
            })
            .collect())
    }
}

#[async_trait::async_trait]
impl TransformsProvider for TransformsProviderService {
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), ServiceError> {
        self.data.list_transform_summaries(offset, limit).await.map_err(ServiceError::from)
    }

    async fn get_transforms_for_workspace_and_user(&self, user_id: UserId, workspace_id: WorkspaceId) -> Result<Vec<DbTransform>, ServiceError> {
        self.data.get_transforms_for_workspace_and_user(user_id, workspace_id).await.map_err(ServiceError::from)
    }

    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, ServiceError> {
        self.data.get_transform(id).await.map_err(ServiceError::from)
    }

    async fn get_transform_draft(&self, id: TransformDraftId) -> Result<DbTransformDraft, ServiceError> {
        self.data.get_transform_draft(id).await.map_err(ServiceError::from)
    }

    async fn get_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, ServiceError> {
        let transforms = self.data.get_transforms(ids).await?;
        let found: std::collections::HashSet<TransformId> = transforms.iter().map(|t| t.transform_id).collect();
        let missing: Vec<TransformId> = ids.iter().copied().filter(|id| !found.contains(id)).collect();
        if missing.is_empty() {
            Ok(transforms)
        } else {
            Err(ServiceError::NotFound)
        }
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

    async fn get_transform_owner(&self, id: TransformId) -> Result<UserId, ServiceError> {
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

    async fn save_transform_draft(&self, id: TransformDraftId, source_code: String, resource_id: Option<ResourceId>) -> Result<DbTransformDraft, ServiceError> {
        self.data.save_transform_draft(id, source_code, resource_id).await.map_err(ServiceError::from)
    }

    /// "Is bucket 2 currently in a publishable state" (source and binary
    /// present and in sync) without mutating anything, so a creator-side
    /// "can I publish?" check can call it without duplicating
    /// `publish_transform`'s own precondition checks. Only meaningful for a
    /// primitive draft — a composite has no source/binary of its own to
    /// check; see `validate_graph_draft`.
    async fn is_draft_publishable(&self, id: TransformDraftId) -> Result<bool, ServiceError> {
        let draft = self.data.get_transform_draft(id).await?;
        Ok(is_bucket2_publishable(&draft))
    }

    async fn check_source(&self, source_code: String) -> Result<(), ServiceError> {
        check_transform_source(&self.build_job_config, &source_code)
            .await
            .map_err(ServiceError::Validation)
    }

    /// See `TransformsProvider::validate_graph_draft`. `id` scopes ownership
    /// only — the graph validated is whatever `metadata_json` the caller
    /// passes, not necessarily what's currently saved, so the Creator can
    /// validate live in-progress edits before saving.
    async fn validate_graph_draft(&self, _id: TransformDraftId, metadata_json: String) -> Result<Vec<PortMetadataJson>, ServiceError> {
        let referenced_ids = serde_json::from_str::<GraphDefinition>(&metadata_json)
            .map(|g| g.referenced_transform_ids())
            .unwrap_or_default();

        let leaf_defs = self.fetch_leaf_defs(&referenced_ids).await?;

        Validator::new()
            .validate(ValidatorInput { metadata_json, leaf_defs })
            .map_err(ServiceError::Validation)
    }

    async fn publish_transform(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError> {
        let draft = self.data.get_transform_draft(id).await?;

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
        if draft.wasm_source_code.as_deref() != Some(draft.source_code.as_str()) {
            return Err(ServiceError::Validation(
                "saved binary no longer corresponds to the saved source; recompile and re-attach before publishing".to_string(),
            ));
        }

        self.data
            .publish_compiled_transform(
                id,
                wasm_bytecode,
                draft.source_code,
                draft.name.unwrap_or_default(),
                draft.description,
                words_to_metadata_string(&draft.metadata),
            )
            .await
            .map_err(ServiceError::from)
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError> {
        self.data.delete_transform(id).await.map_err(ServiceError::from)
    }

    async fn delete_transform_draft(&self, id: TransformDraftId) -> Result<(), ServiceError> {
        self.data.delete_transform_draft(id).await.map_err(ServiceError::from)
    }
}

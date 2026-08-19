use std::sync::Arc;

use domain::db::{
    db_transform::{DbTransform, TransformId},
    db_transform_draft::{DbTransformDraft, TransformDraftId},
    ticket::db_resource::ResourceId,
    WorkspaceId,
};
use domain::domain_user::UserId;
use crate::domain::service_error::ServiceError;

use super::{
    data_provider::transforms_data_provider::TransformsDataProvider,
    transforms_provider::TransformsProvider,
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

pub struct TransformsProviderService {
    data: Arc<dyn TransformsDataProvider>,
}

impl TransformsProviderService {
    pub fn new(data: Arc<dyn TransformsDataProvider>) -> Self {
        Self { data }
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

    /// The name is a holdover from the composite-graph model — see the
    /// task-level note this change was reviewed under. There is no separate
    /// graph to validate any more; this now just answers "is bucket 2
    /// currently in a publishable state" (source and binary present and in
    /// sync) without mutating anything, so a creator-side "can I publish?"
    /// check can still call it without duplicating `publish_transform`'s
    /// own precondition checks.
    async fn validate_composite_draft(&self, id: TransformDraftId) -> Result<bool, ServiceError> {
        let draft = self.data.get_transform_draft(id).await?;
        let publishable = draft
            .wasm_bytecode
            .is_some()
            && draft.wasm_source_code.as_deref() == Some(draft.source_code.as_str());
        Ok(publishable)
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

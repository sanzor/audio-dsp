use std::{collections::HashSet, sync::Arc};

use domain::db::{
    db_transform::{DbTransform, DbTransformBinary, DbTransformDefinition, TransformId},
    ticket::db_resource::ResourceId,
    transform_snapshot::CompositeTransformDefinition,
};
use wasmtime::*;
use crate::domain::service_error::ServiceError;

use super::{
    composite_validator,
    data_provider::transforms_data_provider::{NewTransformParam, NewTransformPort, TransformsDataProvider},
    storage_provider::transform_storage_provider::TransformStorageProvider,
    transforms_provider::{PortShapeSummary, PublishPortShapeDiff, TransformsProvider},
};

/// Every leaf node's `transform_id`, deduplication left to the caller (the
/// data provider's `get_leaf_transform_infos` is keyed lookup, duplicates
/// are harmless). Input/Output nodes have no `transform_id` to collect —
/// `CompositeNode` is a tagged enum as of the Input/Output node model, not a
/// flat struct, so this can no longer be a bare `.map(|n| n.transform_id)`.
fn leaf_transform_ids(graph: &CompositeTransformDefinition) -> Vec<TransformId> {
    graph
        .nodes
        .iter()
        .filter_map(|n| match n {
            domain::db::transform_snapshot::CompositeNode::Leaf { transform_id, .. } => Some(*transform_id),
            _ => None,
        })
        .collect()
}

pub struct TransformsProviderService {
    data: Arc<dyn TransformsDataProvider>,
    storage: Arc<dyn TransformStorageProvider>,
}

impl TransformsProviderService {
    pub fn new(
        data: Arc<dyn TransformsDataProvider>,
        storage: Arc<dyn TransformStorageProvider>,
    ) -> Self {
        Self { data, storage }
    }

    fn collect_missing_ids<T, F>(requested_ids: &[TransformId], items: &[T], get_id: F) -> Vec<TransformId>
    where
        F: Fn(&T) -> TransformId,
    {
        let found: HashSet<TransformId> = items.iter().map(get_id).collect();
        requested_ids
            .iter()
            .copied()
            .filter(|id| !found.contains(id))
            .collect()
    }
}

#[async_trait::async_trait]
impl TransformsProvider for TransformsProviderService {
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), ServiceError> {
        self.data.list_transform_summaries(offset, limit).await.map_err(ServiceError::from)
    }

    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError> {
        self.data.get_transform_definition(id).await.map_err(ServiceError::from)
    }

    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, ServiceError> {
        let definitions = self.data.get_transform_definitions(ids).await?;
        let missing_ids = Self::collect_missing_ids(ids, &definitions, |definition| definition.transform_id);
        if missing_ids.is_empty() {
            Ok(definitions)
        } else {
            // ServiceError::NotFound(format!("Transforms not found: {:?}", missing_ids))
            Err(ServiceError::NotFound)
        }
    }

    async fn get_transform_binary(&self, id: TransformId) -> Result<Vec<u8>, ServiceError> {
        self.storage.get_transform_binary(id).await.map_err(ServiceError::from)
    }

    async fn get_transform_binaries(&self, ids: &[TransformId]) -> Result<Vec<DbTransformBinary>, ServiceError> {
        let binaries = self.storage.get_transform_binaries(ids).await?;
        let missing_ids = Self::collect_missing_ids(ids, &binaries, |binary| binary.transform_id);
        if missing_ids.is_empty() {
            Ok(binaries)
        } else {
             Err(ServiceError::NotFound)
        }
    }

    async fn create_transform(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
        kind: String,
    ) -> Result<DbTransformDefinition, ServiceError> {
        let db = self.data.insert_transform(name, description, icon, kind).await?;
        self.data.get_transform_definition(db.transform_id).await.map_err(ServiceError::from)
    }

    async fn save_transform_draft(&self, id: TransformId, source_code: String, resource_id: Option<ResourceId>) -> Result<DbTransformDefinition, ServiceError> {
        self.data.save_transform_draft(id, source_code, resource_id).await?;
        self.data.get_transform_definition(id).await.map_err(ServiceError::from)
    }

    async fn save_composite_draft(&self, id: TransformId, graph: CompositeTransformDefinition) -> Result<DbTransformDefinition, ServiceError> {
        self.data.save_composite_draft(id, graph).await?;
        self.data.get_transform_definition(id).await.map_err(ServiceError::from)
    }

    async fn validate_composite_draft(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError> {
        let draft = self.data.get_draft(id).await?;
        let Some(graph) = draft.graph_definition else {
            return Err(ServiceError::Validation(
                "nothing has been saved for this composite yet".to_string(),
            ));
        };

        let transform_ids: Vec<TransformId> = leaf_transform_ids(&graph);
        let leaf_defs = self.data.get_leaf_transform_infos(&transform_ids).await?;
        let ports = composite_validator::validate_composite_graph(&graph, &leaf_defs)
            .map_err(ServiceError::Validation)?;

        self.data.validate_composite_draft(id, ports).await?;
        self.data.get_transform_definition(id).await.map_err(ServiceError::from)
    }

    async fn publish_transform(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError> {
        let transform = self.data.get_transform(id).await.map_err(ServiceError::Internal)?;

        if transform.kind == "composite" {
            let draft = self.data.get_draft(id).await?;
            let Some(graph) = draft.graph_definition else {
                return Err(ServiceError::Validation(
                    "nothing has been saved for this composite yet".to_string(),
                ));
            };

            // Gate: the explicit validate action must have succeeded against
            // the currently-persisted graph before Publish will even attempt
            // it. save_composite_draft unconditionally resets is_validated to
            // false on every graph edit, so this can't be stale-true against
            // unsaved canvas changes. See
            // agents/decisions/0008-publish-requires-validated-composite-draft.md
            // (supersedes item 4 of 0007, which had Publish independent of
            // is_validated).
            if !draft.is_validated {
                return Err(ServiceError::Validation(
                    "composite draft must be validated before publishing".to_string(),
                ));
            }

            // Re-validate at publish time, not just at save time — a leaf
            // transform referenced by this graph may have been unpublished
            // or deleted since the last save.
            let transform_ids: Vec<TransformId> = leaf_transform_ids(&graph);
            let leaf_defs = self.data.get_leaf_transform_infos(&transform_ids).await?;
            let ports = composite_validator::validate_composite_graph(&graph, &leaf_defs)
                .map_err(ServiceError::Validation)?;

            self.data
                .publish_composite_transform(id, draft.name.unwrap_or_default(), draft.description, ports, graph)
                .await?;

            return self.data.get_transform_definition(id).await.map_err(ServiceError::from);
        }

        let draft = self.data.get_draft(id).await?;
        let Some(wasm_bytecode) = draft.wasm_bytecode else {
            return Err(ServiceError::Validation(
                "nothing has been saved with a successful build yet".to_string(),
            ));
        };

        // Defense-in-depth: save_transform_draft's attach branch now checks
        // provenance at write time, so a save can no longer *create* a
        // mismatched pair. But a source-only save after an earlier attach
        // moves source_code forward while leaving wasm_bytecode (and this
        // wasm_source_code snapshot) pointing at the older text — that's the
        // real, still-reachable drift this guards against. See
        // agents/decisions/0002-transform-draft-lifecycle-decisions.md.
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
                draft.ports.into_iter().map(NewTransformPort::from).collect(),
                draft.params.into_iter().map(NewTransformParam::from).collect(),
            )
            .await?;

        self.data.get_transform_definition(id).await.map_err(ServiceError::from)
    }

    async fn diff_publish_port_shape(&self, id: TransformId) -> Result<PublishPortShapeDiff, ServiceError> {
        let current_ports = self.data.get_current_ports(id).await?;

        // Never been published — nothing to diff against, so nothing to
        // warn about. Every published transform has at least one output
        // port (introspection enforces exactly one), so an empty current
        // set unambiguously means "no prior publish".
        if current_ports.is_empty() {
            return Ok(PublishPortShapeDiff { changed: false, current: vec![], incoming: vec![] });
        }

        let draft = self.data.get_draft(id).await?;

        let current: Vec<PortShapeSummary> = current_ports
            .into_iter()
            .map(|p| PortShapeSummary { name: p.name, direction: p.direction, kind: p.kind, cardinality: p.cardinality })
            .collect();
        let incoming: Vec<PortShapeSummary> = draft
            .ports
            .into_iter()
            .map(|p| PortShapeSummary { name: p.name, direction: p.direction, kind: p.kind, cardinality: p.cardinality })
            .collect();

        // Shape means count/kind/cardinality per port, not identity — a pure
        // rename with everything else unchanged shouldn't trip the warning,
        // matching the feature brief's "count/kind/cardinality per port".
        let shape_of = |ports: &[PortShapeSummary]| -> Vec<(String, String, String)> {
            let mut shape: Vec<(String, String, String)> = ports
                .iter()
                .map(|p| (p.direction.clone(), p.kind.clone(), p.cardinality.clone()))
                .collect();
            shape.sort();
            shape
        };
        let changed = shape_of(&current) != shape_of(&incoming);

        Ok(PublishPortShapeDiff { changed, current, incoming })
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError> {
        self.data.delete_transform(id).await.map_err(ServiceError::from)
    }
}

#[cfg(test)]
#[path = "transforms_provider_service_tests.rs"]
mod tests;

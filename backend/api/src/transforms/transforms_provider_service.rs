use std::{collections::HashSet, sync::Arc};

use domain::db::{
    db_transform::{DbTransform, DbTransformBinary, DbTransformDefinition, TransformId},
    ticket::db_resource::ResourceId,
    transform_snapshot::CompositeGraphDefinition,
};
use wasmtime::*;
use crate::domain::service_error::ServiceError;

use super::{
    composite_validator,
    data_provider::transforms_data_provider::{NewTransformParam, NewTransformPort, TransformsDataProvider},
    storage_provider::transform_storage_provider::TransformStorageProvider,
    transforms_provider::{PortShapeSummary, PublishPortShapeDiff, TransformsProvider},
};

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

    async fn save_composite_draft(&self, id: TransformId, graph: CompositeGraphDefinition) -> Result<DbTransformDefinition, ServiceError> {
        let transform_ids: Vec<TransformId> = graph.nodes.iter().map(|n| n.transform_id).collect();
        let leaf_defs = self.data.get_leaf_transform_infos(&transform_ids).await?;
        let ports = composite_validator::validate_composite_graph(&graph, &leaf_defs)
            .map_err(ServiceError::Validation)?;

        self.data.save_composite_draft(id, graph, ports).await?;
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

            // Re-validate at publish time, not just at save time — a leaf
            // transform referenced by this graph may have been unpublished
            // or deleted since the last save.
            let transform_ids: Vec<TransformId> = graph.nodes.iter().map(|n| n.transform_id).collect();
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
mod tests {
    //! `publish_transform`'s source/binary consistency gate (item 2 of the
    //! transform-draft-lifecycle follow-ups, see
    //! agents/decisions/0002-transform-draft-lifecycle-decisions.md) is pure
    //! Rust logic over whatever `get_draft` returns — no DB needed to
    //! exercise it. There's no DB-backed test harness in this repo yet (the
    //! only other transform test, transform_compile_pipeline.rs, drives the
    //! real compiler/wasmtime and is `#[ignore]`d), so this uses a minimal
    //! in-memory `TransformsDataProvider` double instead. Anything the
    //! publish path doesn't touch panics with `unimplemented!()` so a test
    //! relying on unexercised behavior fails loudly rather than silently
    //! doing the wrong thing.
    use super::*;
    use domain::db::{
        db_transform::DbTransformDefinition,
        ticket::{
            create_ticket_params::CreateTicketParams, db_resource::DbResource,
            db_ticket::{DbTicket, TicketId},
            update_ticket_params::UpdateTicketParams,
        },
        db_transform_draft::DbTransformDraft,
    };

    struct FakeDataProvider {
        draft: DbTransformDraft,
        current_ports: Vec<domain::db::db_transform::DbTransformPort>,
    }

    #[async_trait::async_trait]
    impl TransformsDataProvider for FakeDataProvider {
        async fn create_ticket(&self, _: CreateTicketParams) -> Result<DbTicket, crate::domain::data_error::DataError> { unimplemented!() }
        async fn get_ticket(&self, _: TicketId) -> Result<DbTicket, crate::domain::data_error::DataError> { unimplemented!() }
        async fn create_resource(&self, _: TicketId, _: Vec<u8>, _: String, _: Option<String>, _: Vec<NewTransformPort>, _: Vec<NewTransformParam>) -> Result<DbResource, crate::domain::data_error::DataError> { unimplemented!() }
        async fn get_resource(&self, _: ResourceId) -> Result<DbResource, crate::domain::data_error::DataError> { unimplemented!() }
        async fn update_resource(&self, _: ResourceId, _: TicketId) -> Result<DbResource, crate::domain::data_error::DataError> { unimplemented!() }
        async fn remove_resource(&self, _: ResourceId) -> Result<(), crate::domain::data_error::DataError> { unimplemented!() }
        async fn remove_ticket(&self, _: TicketId) -> Result<(), crate::domain::data_error::DataError> { unimplemented!() }
        async fn update_ticket(&self, _: UpdateTicketParams) -> Result<DbTicket, crate::domain::data_error::DataError> { unimplemented!() }
        async fn get_transform(&self, _: TransformId) -> Result<DbTransform, String> { unimplemented!() }
        async fn get_current_ports(&self, _: TransformId) -> Result<Vec<domain::db::db_transform::DbTransformPort>, crate::domain::data_error::DataError> { Ok(self.current_ports.clone()) }

        async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, String> {
            Ok(DbTransformDefinition {
                transform_id: id,
                name: self.draft.name.clone().unwrap_or_default(),
                description: self.draft.description.clone(),
                icon: None,
                kind: "primitive".to_string(),
                source_code: Some(self.draft.source_code.clone()),
                graph_definition: None,
                ports: vec![],
                params: vec![],
            })
        }

        async fn get_transform_definitions(&self, _: &[TransformId]) -> Result<Vec<DbTransformDefinition>, String> { unimplemented!() }
        async fn list_transform_summaries(&self, _: i64, _: i64) -> Result<(Vec<DbTransform>, i64), String> { unimplemented!() }
        async fn insert_transform(&self, _: String, _: Option<String>, _: Option<String>, _: String) -> Result<DbTransform, String> { unimplemented!() }
        async fn delete_transform(&self, _: TransformId) -> Result<(), crate::domain::data_error::DataError> { unimplemented!() }

        async fn get_leaf_transform_infos(&self, _: &[TransformId]) -> Result<std::collections::HashMap<TransformId, crate::transforms::composite_validator::LeafTransformInfo>, crate::domain::data_error::DataError> { unimplemented!() }

        async fn get_draft(&self, _: TransformId) -> Result<DbTransformDraft, crate::domain::data_error::DataError> {
            Ok(self.draft.clone())
        }

        async fn save_transform_draft(&self, _: TransformId, _: String, _: Option<ResourceId>) -> Result<DbTransformDraft, crate::domain::data_error::DataError> { unimplemented!() }
        async fn save_composite_draft(&self, _: TransformId, _: domain::db::transform_snapshot::CompositeGraphDefinition, _: Vec<NewTransformPort>) -> Result<DbTransformDraft, crate::domain::data_error::DataError> { unimplemented!() }

        async fn publish_compiled_transform(&self, _: TransformId, _: Vec<u8>, _: String, _: String, _: Option<String>, _: Vec<NewTransformPort>, _: Vec<NewTransformParam>) -> Result<(), String> {
            Ok(())
        }

        async fn publish_composite_transform(&self, _: TransformId, _: String, _: Option<String>, _: Vec<NewTransformPort>, _: domain::db::transform_snapshot::CompositeGraphDefinition) -> Result<(), String> { unimplemented!() }
    }

    struct FakeStorageProvider;

    #[async_trait::async_trait]
    impl TransformStorageProvider for FakeStorageProvider {
        async fn get_transform_binary(&self, _: TransformId) -> Result<Vec<u8>, String> { unimplemented!() }
        async fn get_transform_binaries(&self, _: &[TransformId]) -> Result<Vec<DbTransformBinary>, String> { unimplemented!() }
        async fn write_transform_binary(&self, _: TransformId, _: &[u8]) -> Result<(), String> { unimplemented!() }
    }

    fn make_draft(source_code: &str, wasm_source_code: Option<&str>, has_binary: bool) -> DbTransformDraft {
        DbTransformDraft {
            transform_id: 1,
            source_code: source_code.to_string(),
            wasm_bytecode: if has_binary { Some(vec![0, 1, 2]) } else { None },
            wasm_source_code: wasm_source_code.map(|s| s.to_string()),
            graph_definition: None,
            name: Some("Test".to_string()),
            description: None,
            ports: vec![],
            params: vec![],
        }
    }

    fn service_with(draft: DbTransformDraft) -> TransformsProviderService {
        service_with_ports(draft, vec![])
    }

    fn service_with_ports(
        draft: DbTransformDraft,
        current_ports: Vec<domain::db::db_transform::DbTransformPort>,
    ) -> TransformsProviderService {
        TransformsProviderService::new(
            Arc::new(FakeDataProvider { draft, current_ports }),
            Arc::new(FakeStorageProvider),
        )
    }

    fn db_port(name: &str, direction: &str, kind: &str, cardinality: &str) -> domain::db::db_transform::DbTransformPort {
        domain::db::db_transform::DbTransformPort {
            port_id: 1,
            transform_id: 1,
            name: name.to_string(),
            direction: direction.to_string(),
            port_order: 0,
            description: None,
            kind: kind.to_string(),
            cardinality: cardinality.to_string(),
        }
    }

    fn port_snapshot(name: &str, direction: &str, kind: &str, cardinality: &str) -> domain::db::transform_snapshot::PortSnapshot {
        domain::db::transform_snapshot::PortSnapshot {
            name: name.to_string(),
            direction: direction.to_string(),
            port_order: 0,
            description: None,
            kind: kind.to_string(),
            cardinality: cardinality.to_string(),
        }
    }

    #[tokio::test]
    async fn publish_fails_when_saved_binary_no_longer_matches_saved_source() {
        // Models: resource attached while source was "v1" (item 1 stamps
        // wasm_source_code = "v1" at that moment), then a later source-only
        // save moves source_code to "v2" without touching the binary. This
        // is the drift item 2's gate exists to catch.
        let service = service_with(make_draft("v2", Some("v1"), true));

        let result = service.publish_transform(1).await;

        assert!(
            matches!(result, Err(ServiceError::Validation(_))),
            "expected a validation error on source/binary drift, got {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn publish_succeeds_when_saved_binary_matches_saved_source() {
        let service = service_with(make_draft("v1", Some("v1"), true));

        let result = service.publish_transform(1).await;

        assert!(result.is_ok(), "expected publish to succeed, got {:?}", result.err());
    }

    #[tokio::test]
    async fn publish_fails_when_nothing_saved_with_a_binary_yet() {
        let service = service_with(make_draft("v1", None, false));

        let result = service.publish_transform(1).await;

        assert!(matches!(result, Err(ServiceError::Validation(_))));
    }

    #[tokio::test]
    async fn port_shape_diff_reports_no_change_when_never_published() {
        // No current_ports rows at all — first-ever publish, nothing to
        // warn about regardless of what's saved.
        let mut draft = make_draft("v1", Some("v1"), true);
        draft.ports = vec![port_snapshot("a", "input", "program", "single"), port_snapshot("out", "output", "program", "single")];
        let service = service_with_ports(draft, vec![]);

        let diff = service.diff_publish_port_shape(1).await.expect("diff should succeed");

        assert!(!diff.changed);
    }

    #[tokio::test]
    async fn port_shape_diff_flags_a_1_to_2_input_republish() {
        let current = vec![db_port("in", "input", "program", "single"), db_port("out", "output", "program", "single")];
        let mut draft = make_draft("v1", Some("v1"), true);
        draft.ports = vec![
            port_snapshot("a", "input", "program", "single"),
            port_snapshot("b", "input", "sidechain", "single"),
            port_snapshot("out", "output", "program", "single"),
        ];
        let service = service_with_ports(draft, current);

        let diff = service.diff_publish_port_shape(1).await.expect("diff should succeed");

        assert!(diff.changed, "expected a 1-input -> 2-input republish to be flagged");
        assert_eq!(diff.current.len(), 2);
        assert_eq!(diff.incoming.len(), 3);
    }

    #[tokio::test]
    async fn port_shape_diff_ignores_a_pure_rename() {
        let current = vec![db_port("in", "input", "program", "single"), db_port("out", "output", "program", "single")];
        let mut draft = make_draft("v1", Some("v1"), true);
        // Same shape (direction/kind/cardinality), different name only.
        draft.ports = vec![
            port_snapshot("input_signal", "input", "program", "single"),
            port_snapshot("out", "output", "program", "single"),
        ];
        let service = service_with_ports(draft, current);

        let diff = service.diff_publish_port_shape(1).await.expect("diff should succeed");

        assert!(!diff.changed, "a pure rename with unchanged shape should not be flagged");
    }
}

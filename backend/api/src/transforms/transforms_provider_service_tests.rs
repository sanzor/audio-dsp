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
    transform_snapshot::{CompositeEdge, CompositeTransformDefinition, CompositeNode, CompositeNodePosition},
};

/// Plain-data stand-in for `composite_validator::LeafTransformInfo` (which
/// isn't `Clone`) so `FakeDataProvider::leaf_defs` can be a `HashMap` that
/// `get_leaf_transform_infos` builds real `LeafTransformInfo`s from on each
/// call, keyed by whatever ids the composite branch asks for.
#[derive(Clone)]
struct LeafSpec {
    kind: String,
    published: bool,
    ports: Vec<domain::db::db_transform::DbTransformPort>,
}

struct FakeDataProvider {
    draft: DbTransformDraft,
    current_ports: Vec<domain::db::db_transform::DbTransformPort>,
    /// "primitive" | "composite" — drives `get_transform`'s `.kind`, which
    /// `publish_transform` branches on before anything else.
    kind: String,
    leaf_defs: std::collections::HashMap<TransformId, LeafSpec>,
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

    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String> {
        Ok(DbTransform {
            transform_id: id,
            name: self.draft.name.clone().unwrap_or_default(),
            description: self.draft.description.clone(),
            icon: None,
            kind: self.kind.clone(),
            published: false,
            created_at: chrono::Utc::now(),
        })
    }

    async fn get_current_ports(&self, _: TransformId) -> Result<Vec<domain::db::db_transform::DbTransformPort>, crate::domain::data_error::DataError> { Ok(self.current_ports.clone()) }

    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, String> {
        Ok(DbTransformDefinition {
            transform_id: id,
            name: self.draft.name.clone().unwrap_or_default(),
            description: self.draft.description.clone(),
            icon: None,
            kind: self.kind.clone(),
            source_code: Some(self.draft.source_code.clone()),
            graph_definition: None,
            is_validated: self.draft.is_validated,
            ports: vec![],
            params: vec![],
        })
    }

    async fn get_transform_definitions(&self, _: &[TransformId]) -> Result<Vec<DbTransformDefinition>, String> { unimplemented!() }
    async fn list_transform_summaries(&self, _: i64, _: i64) -> Result<(Vec<DbTransform>, i64), String> { unimplemented!() }
    async fn insert_transform(&self, _: String, _: Option<String>, _: Option<String>, _: String) -> Result<DbTransform, String> { unimplemented!() }
    async fn delete_transform(&self, _: TransformId) -> Result<(), crate::domain::data_error::DataError> { unimplemented!() }

    async fn get_leaf_transform_infos(&self, ids: &[TransformId]) -> Result<std::collections::HashMap<TransformId, crate::transforms::composite_validator::LeafTransformInfo>, crate::domain::data_error::DataError> {
        Ok(ids
            .iter()
            .filter_map(|id| {
                self.leaf_defs.get(id).map(|spec| {
                    (
                        *id,
                        crate::transforms::composite_validator::LeafTransformInfo {
                            kind: spec.kind.clone(),
                            published: spec.published,
                            ports: spec.ports.clone(),
                        },
                    )
                })
            })
            .collect())
    }

    async fn get_draft(&self, _: TransformId) -> Result<DbTransformDraft, crate::domain::data_error::DataError> {
        Ok(self.draft.clone())
    }

    async fn save_transform_draft(&self, _: TransformId, _: String, _: Option<ResourceId>) -> Result<DbTransformDraft, crate::domain::data_error::DataError> { unimplemented!() }
    async fn save_composite_draft(&self, _: TransformId, _: domain::db::transform_snapshot::CompositeTransformDefinition) -> Result<DbTransformDraft, crate::domain::data_error::DataError> { unimplemented!() }
    async fn validate_composite_draft(&self, _: TransformId, _: Vec<NewTransformPort>) -> Result<DbTransformDraft, crate::domain::data_error::DataError> { unimplemented!() }

    async fn publish_compiled_transform(&self, _: TransformId, _: Vec<u8>, _: String, _: String, _: Option<String>, _: Vec<NewTransformPort>, _: Vec<NewTransformParam>) -> Result<(), String> {
        Ok(())
    }

    async fn publish_composite_transform(&self, _: TransformId, _: String, _: Option<String>, _: Vec<NewTransformPort>, _: domain::db::transform_snapshot::CompositeTransformDefinition) -> Result<(), String> {
        Ok(())
    }
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
        is_validated: false,
    }
}

/// A minimal but structurally valid composite draft: Input -> leaf -> Output
/// (node ids 1/2/3), wired so `composite_validator::validate_composite_graph`
/// succeeds outright when the matching `leaf_defs` (see
/// `single_leaf_defs` below) are supplied to `service_with_composite`. Used
/// by the `is_validated` publish-gate tests (0008) — only `is_validated`
/// varies between them, the graph itself never has to fail validation.
fn make_composite_draft(is_validated: bool) -> DbTransformDraft {
    let graph = CompositeTransformDefinition {
        nodes: vec![
            CompositeNode::Input { node_id: 1, name: "in".to_string(), position: CompositeNodePosition::default() },
            CompositeNode::Leaf { node_id: 2, transform_id: 99, position: CompositeNodePosition::default() },
            CompositeNode::Output { node_id: 3, name: "out".to_string(), position: CompositeNodePosition::default() },
        ],
        edges: vec![
            CompositeEdge { from_node_id: 1, from_port: "signal".to_string(), to_node_id: 2, to_port: "in".to_string() },
            CompositeEdge { from_node_id: 2, from_port: "out".to_string(), to_node_id: 3, to_port: "signal".to_string() },
        ],
    };
    DbTransformDraft {
        transform_id: 1,
        source_code: String::new(),
        wasm_bytecode: None,
        wasm_source_code: None,
        graph_definition: Some(graph),
        name: Some("Test Composite".to_string()),
        description: None,
        ports: vec![],
        params: vec![],
        is_validated,
    }
}

/// `leaf_defs` matching `make_composite_draft`'s leaf node (transform_id 99):
/// a published primitive with an "in"/"out" program-single port pair, enough
/// for `validate_composite_graph` to succeed when the publish-gate check
/// lets it run at all.
fn single_leaf_defs() -> std::collections::HashMap<TransformId, LeafSpec> {
    let mut defs = std::collections::HashMap::new();
    defs.insert(
        99,
        LeafSpec {
            kind: "primitive".to_string(),
            published: true,
            ports: vec![
                db_port("in", "input", "program", "single"),
                db_port("out", "output", "program", "single"),
            ],
        },
    );
    defs
}

fn service_with(draft: DbTransformDraft) -> TransformsProviderService {
    service_with_ports(draft, vec![])
}

fn service_with_ports(
    draft: DbTransformDraft,
    current_ports: Vec<domain::db::db_transform::DbTransformPort>,
) -> TransformsProviderService {
    TransformsProviderService::new(
        Arc::new(FakeDataProvider { draft, current_ports, kind: "primitive".to_string(), leaf_defs: std::collections::HashMap::new() }),
        Arc::new(FakeStorageProvider),
    )
}

fn service_with_composite(
    draft: DbTransformDraft,
    leaf_defs: std::collections::HashMap<TransformId, LeafSpec>,
) -> TransformsProviderService {
    TransformsProviderService::new(
        Arc::new(FakeDataProvider { draft, current_ports: vec![], kind: "composite".to_string(), leaf_defs }),
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
async fn publish_fails_when_composite_draft_not_validated() {
    // 0008: publish's composite branch now gates on is_validated before it
    // even attempts re-validation. The graph here is otherwise perfectly
    // valid (see make_composite_draft/single_leaf_defs) — the only thing
    // wrong is is_validated: false — so a failure here can only be coming
    // from the new gate, not from validate_composite_graph itself.
    let service = service_with_composite(make_composite_draft(false), single_leaf_defs());

    let result = service.publish_transform(1).await;

    match result {
        Err(ServiceError::Validation(msg)) => {
            assert!(
                msg.contains("validated"),
                "expected the not-validated gate's message, got: {msg:?}"
            );
        }
        other => panic!("expected ServiceError::Validation from the is_validated gate, got {other:?}"),
    }
}

#[tokio::test]
async fn publish_succeeds_when_composite_draft_is_validated() {
    // is_validated: true clears the new gate, and the existing
    // re-validation logic (leaf lookup + validate_composite_graph) still
    // runs afterward and must itself succeed against single_leaf_defs's
    // published leaf — this exercises both checks coexisting, not just the
    // gate in isolation.
    let service = service_with_composite(make_composite_draft(true), single_leaf_defs());

    let result = service.publish_transform(1).await;

    assert!(result.is_ok(), "expected publish to succeed, got {:?}", result.err());
}

#[tokio::test]
async fn publish_fails_when_composite_draft_is_validated_but_leaf_no_longer_published() {
    // is_validated: true clears the new gate, but the re-validation that
    // runs afterward must still independently catch a leaf that was
    // unpublished/deleted since the last save/validate — confirming the
    // gate doesn't weaken or replace that existing check (0007's original
    // reasoning, still true post-0008).
    let mut leaf_defs = single_leaf_defs();
    leaf_defs.get_mut(&99).unwrap().published = false;
    let service = service_with_composite(make_composite_draft(true), leaf_defs);

    let result = service.publish_transform(1).await;

    assert!(
        matches!(result, Err(ServiceError::Validation(_))),
        "expected re-validation to still catch an unpublished leaf, got {:?}",
        result.err()
    );
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

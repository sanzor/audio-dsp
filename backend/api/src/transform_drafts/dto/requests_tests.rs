use super::{SaveDraftParams, SavePrimitiveParams};

#[test]
fn save_primitive_payload_accepts_source_code() {
    let payload: SaveDraftParams =
        serde_json::from_str(r#"{"source_code":"impl Transform for Gain {}"}"#)
            .expect("primitive save payload should deserialize");

    match payload {
        SaveDraftParams::Primitive(SavePrimitiveParams { source_code }) => {
            assert_eq!(source_code, "impl Transform for Gain {}");
        }
        SaveDraftParams::Composite(_) => panic!("expected primitive payload"),
    }
}

#[test]
fn save_composite_payload_accepts_graph_definition() {
    let payload: SaveDraftParams =
        serde_json::from_str(r#"{"graph_definition":{"nodes":[],"edges":[]}}"#)
            .expect("composite save payload should deserialize");

    assert!(matches!(payload, SaveDraftParams::Composite(_)));
}

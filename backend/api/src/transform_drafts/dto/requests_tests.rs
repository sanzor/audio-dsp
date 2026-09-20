use super::{SaveCompositeParams, SaveDraftParams, SavePrimitiveParams};

#[test]
fn save_primitive_payload_accepts_source_code() {
    let payload: SaveDraftParams =
        serde_json::from_str(r#"{"source_code":"impl Transform for Gain {}"}"#)
            .expect("primitive save payload should deserialize");

    match payload {
        SaveDraftParams::Primitive(SavePrimitiveParams {
            source_code,
            name,
            description,
        }) => {
            assert_eq!(source_code, "impl Transform for Gain {}");
            assert_eq!(name, None);
            assert_eq!(description, None);
        }
        SaveDraftParams::Composite(_) => panic!("expected primitive payload"),
    }
}

#[test]
fn save_composite_payload_accepts_graph_definition() {
    let payload: SaveDraftParams =
        serde_json::from_str(r#"{"graph_definition":{"nodes":[],"edges":[]}}"#)
            .expect("composite save payload should deserialize");

    match payload {
        SaveDraftParams::Composite(SaveCompositeParams { name, description, .. }) => {
            assert_eq!(name, None);
            assert_eq!(description, None);
        }
        SaveDraftParams::Primitive(_) => panic!("expected composite payload"),
    }
}

#[test]
fn save_primitive_payload_accepts_optional_name_and_description() {
    let payload: SaveDraftParams = serde_json::from_str(
        r#"{"source_code":"impl Transform for Gain {}","name":"Gain","description":"boosts signal"}"#,
    )
    .expect("primitive save payload with name/description should deserialize");

    match payload {
        SaveDraftParams::Primitive(SavePrimitiveParams { name, description, .. }) => {
            assert_eq!(name.as_deref(), Some("Gain"));
            assert_eq!(description.as_deref(), Some("boosts signal"));
        }
        SaveDraftParams::Composite(_) => panic!("expected primitive payload"),
    }
}

#[test]
fn save_primitive_payload_allows_null_description() {
    let payload: SaveDraftParams = serde_json::from_str(
        r#"{"source_code":"impl Transform for Gain {}","name":"Gain","description":null}"#,
    )
    .expect("payload should deserialize");

    match payload {
        SaveDraftParams::Primitive(SavePrimitiveParams { name, description, .. }) => {
            assert_eq!(name.as_deref(), Some("Gain"));
            assert_eq!(description, None);
        }
        SaveDraftParams::Composite(_) => panic!("expected primitive payload"),
    }
}

#[test]
fn save_composite_payload_accepts_optional_name_and_description() {
    let payload: SaveDraftParams = serde_json::from_str(
        r#"{"graph_definition":{"nodes":[],"edges":[]},"name":"Chain","description":"a chain"}"#,
    )
    .expect("composite save payload with name/description should deserialize");

    match payload {
        SaveDraftParams::Composite(SaveCompositeParams { name, description, .. }) => {
            assert_eq!(name.as_deref(), Some("Chain"));
            assert_eq!(description.as_deref(), Some("a chain"));
        }
        SaveDraftParams::Primitive(_) => panic!("expected composite payload"),
    }
}

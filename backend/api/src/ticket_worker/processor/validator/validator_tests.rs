use crate::ticket_worker::processor::{transform_metadata::{DirectionJson, PortCardinalityJson, PortKindJson, PortMetadataJson}, wasm::wasm_parser::{ParsedPrimitiveWasm, PrimitiveMetadataJson}};
use super::validate_primitive;

fn program_port(name: &str, direction: DirectionJson, order: i32) -> PortMetadataJson {
    PortMetadataJson {
        name: name.to_string(),
        direction,
        order,
        description: None,
        kind: PortKindJson::Program,
        cardinality: PortCardinalityJson::Single,
    }
}

fn base_metadata(ports: Vec<PortMetadataJson>) -> PrimitiveMetadataJson {
    PrimitiveMetadataJson {
        name: "Test".to_string(),
        description: None,
        ports,
        params: vec![],
    }
}

fn parsed(metadata: PrimitiveMetadataJson, has_abi_version: bool) -> ParsedPrimitiveWasm {
    ParsedPrimitiveWasm {
        metadata,
        has_abi_version,
    }
}
#[test]
fn accepts_a_single_input_single_output_transform_with_abi_version() {
    let metadata = base_metadata(vec![
        program_port("in", DirectionJson::Input, 0),
        program_port("out", DirectionJson::Output, 0),
    ]);
    assert!(validate_primitive(parsed(metadata, true)).is_ok());
}

#[test]
fn accepts_multi_input_when_abi_version_is_present() {
    let metadata = base_metadata(vec![
        program_port("a", DirectionJson::Input, 0),
        program_port("b", DirectionJson::Input, 1),
        program_port("out", DirectionJson::Output, 0),
    ]);
    assert!(validate_primitive(parsed(metadata, true)).is_ok());
}

#[test]
fn rejects_multi_input_without_abi_version() {
    let metadata = base_metadata(vec![
        program_port("a", DirectionJson::Input, 0),
        program_port("b", DirectionJson::Input, 1),
        program_port("out", DirectionJson::Output, 0),
    ]);
    let err = validate_primitive(parsed(metadata, false)).unwrap_err();
    assert!(err.contains("legacy"), "unexpected error: {err}");
}

#[test]
fn rejects_zero_output_ports() {
    let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0)]);
    let err = validate_primitive(parsed(metadata, true)).unwrap_err();
    assert!(
        err.contains("exactly one output port"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_two_output_ports() {
    let metadata = base_metadata(vec![
        program_port("in", DirectionJson::Input, 0),
        program_port("out1", DirectionJson::Output, 0),
        program_port("out2", DirectionJson::Output, 1),
    ]);
    let err = validate_primitive(parsed(metadata, true)).unwrap_err();
    assert!(
        err.contains("exactly one output port"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_sidechain_output_port() {
    let mut output = program_port("out", DirectionJson::Output, 0);
    output.kind = PortKindJson::Sidechain;
    let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0), output]);
    let err = validate_primitive(parsed(metadata, true)).unwrap_err();
    assert!(err.contains("kind=program"), "unexpected error: {err}");
}

#[test]
fn rejects_many_cardinality_output_port() {
    let mut output = program_port("out", DirectionJson::Output, 0);
    output.cardinality = PortCardinalityJson::Many;
    let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0), output]);
    let err = validate_primitive(parsed(metadata, true)).unwrap_err();
    assert!(
        err.contains("cardinality=single"),
        "unexpected error: {err}"
    );
}

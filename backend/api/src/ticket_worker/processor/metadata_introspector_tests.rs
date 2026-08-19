use super::*;

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

fn base_metadata(ports: Vec<PortMetadataJson>) -> TransformMetadataJson {
    TransformMetadataJson {
        name: "Test".to_string(),
        description: None,
        ports,
        params: vec![],
    }
}

#[test]
fn accepts_a_single_input_single_output_transform_with_abi_version() {
    let metadata = base_metadata(vec![
        program_port("in", DirectionJson::Input, 0),
        program_port("out", DirectionJson::Output, 0),
    ]);
    assert!(validate_metadata(&metadata, true).is_ok());
}

#[test]
fn accepts_multi_input_when_abi_version_is_present() {
    let metadata = base_metadata(vec![
        program_port("a", DirectionJson::Input, 0),
        program_port("b", DirectionJson::Input, 1),
        program_port("out", DirectionJson::Output, 0),
    ]);
    assert!(validate_metadata(&metadata, true).is_ok());
}

#[test]
fn rejects_multi_input_without_abi_version() {
    let metadata = base_metadata(vec![
        program_port("a", DirectionJson::Input, 0),
        program_port("b", DirectionJson::Input, 1),
        program_port("out", DirectionJson::Output, 0),
    ]);
    let err = validate_metadata(&metadata, false).unwrap_err();
    assert!(err.contains("legacy"), "unexpected error: {err}");
}

#[test]
fn rejects_zero_output_ports() {
    let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0)]);
    let err = validate_metadata(&metadata, true).unwrap_err();
    assert!(err.contains("exactly one output port"), "unexpected error: {err}");
}

#[test]
fn rejects_two_output_ports() {
    let metadata = base_metadata(vec![
        program_port("in", DirectionJson::Input, 0),
        program_port("out1", DirectionJson::Output, 0),
        program_port("out2", DirectionJson::Output, 1),
    ]);
    let err = validate_metadata(&metadata, true).unwrap_err();
    assert!(err.contains("exactly one output port"), "unexpected error: {err}");
}

#[test]
fn rejects_sidechain_output_port() {
    let mut output = program_port("out", DirectionJson::Output, 0);
    output.kind = PortKindJson::Sidechain;
    let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0), output]);
    let err = validate_metadata(&metadata, true).unwrap_err();
    assert!(err.contains("kind=program"), "unexpected error: {err}");
}

#[test]
fn rejects_many_cardinality_output_port() {
    let mut output = program_port("out", DirectionJson::Output, 0);
    output.cardinality = PortCardinalityJson::Many;
    let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0), output]);
    let err = validate_metadata(&metadata, true).unwrap_err();
    assert!(err.contains("cardinality=single"), "unexpected error: {err}");
}

#[test]
fn rejects_duplicate_port_names_within_a_direction() {
    let metadata = base_metadata(vec![
        program_port("in", DirectionJson::Input, 0),
        program_port("in", DirectionJson::Input, 1),
        program_port("out", DirectionJson::Output, 0),
    ]);
    let err = validate_metadata(&metadata, true).unwrap_err();
    assert!(err.contains("duplicate input port name"), "unexpected error: {err}");
}

#[test]
fn allows_same_name_across_directions() {
    let metadata = base_metadata(vec![
        program_port("main", DirectionJson::Input, 0),
        program_port("main", DirectionJson::Output, 0),
    ]);
    assert!(validate_metadata(&metadata, true).is_ok());
}

#[test]
fn accepts_sidechain_input_port_alongside_one_program_input() {
    let mut sidechain = program_port("key", DirectionJson::Input, 1);
    sidechain.kind = PortKindJson::Sidechain;
    let metadata = base_metadata(vec![
        program_port("program", DirectionJson::Input, 0),
        sidechain,
        program_port("out", DirectionJson::Output, 0),
    ]);
    assert!(validate_metadata(&metadata, true).is_ok());
}

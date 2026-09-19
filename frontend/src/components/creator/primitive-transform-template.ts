import type { TransformPort } from "@/domain/Transform/TransformPort";

// The common starting point for every new primitive draft. It deliberately
// has no parameters and preserves the program signal unchanged, so a creator
// starts from a valid, understandable transform rather than a blank file.
//
// This is display-time starter state only. The backend compiles and
// introspects the source on Save; those compiled ports replace this template
// definition once it is available.
export const PRIMITIVE_TRANSFORM_TEMPLATE_SOURCE = `use transform_sdk::{Transform, TransformMetadata, PortMetadata, Direction, PortKind, PortCardinality, Params};

#[derive(Default)]
pub struct Passthrough;

impl Transform for Passthrough {
    fn process(&mut self, samples: &[&[f32]], _params: &Params<'_>) -> Vec<f32> {
        samples[0].to_vec()
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "Passthrough".to_string(),
            description: Some("Passes the input signal through unchanged.".to_string()),
            ports: vec![
                PortMetadata { name: "in".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![],
        }
    }
}

transform_sdk::export_transform!(Passthrough);
`;

// These mirror the template source above. They are used only while a newly
// created primitive has no saved, compile-derived metadata yet.
export const PRIMITIVE_TRANSFORM_TEMPLATE_PORTS: TransformPort[] = [
  {
    port_id: -1,
    name: "in",
    direction: "input",
    port_order: 0,
    kind: "program",
    cardinality: "single",
  },
  {
    port_id: -2,
    name: "out",
    direction: "output",
    port_order: 0,
    kind: "program",
    cardinality: "single",
  },
];

use transform_sdk::{Direction, ParamMetadata, Params, PortCardinality, PortKind, PortMetadata, Transform, TransformMetadata};

#[derive(Default)]
pub struct Mixer;

impl Transform for Mixer {
    fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32> {
        let output_gain = params[0].clamp(0.0, 2.0);
        let a = samples[0];
        let b = samples[1];

        a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| (x + y) * output_gain)
            .collect()
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "Mixer".to_string(),
            description: Some("Applies output trim after the graph sums upstream inputs.".to_string()),
            ports: vec![
                PortMetadata { name: "In A".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "In B".to_string(), direction: Direction::Input, order: 1, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "Out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![ParamMetadata {
                name: "output_gain".to_string(),
                order: 0,
                default: 1.0,
                min: Some(0.0),
                max: Some(2.0),
                description: Some("Output trim applied after the graph sum.".to_string()),
            }],
        }
    }
}

transform_sdk::export_transform!(Mixer);

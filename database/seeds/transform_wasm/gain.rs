use transform_sdk::{Direction, ParamMetadata, Params, PortCardinality, PortKind, PortMetadata, Transform, TransformMetadata};

#[derive(Default)]
pub struct Gain;

impl Transform for Gain {
    fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32> {
        let gain = params[0].clamp(0.0, 4.0);
        samples[0].iter().map(|s| s * gain).collect()
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "Gain".to_string(),
            description: Some("Amplifies or attenuates the signal level.".to_string()),
            ports: vec![
                PortMetadata { name: "In".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "Out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![ParamMetadata {
                name: "gain".to_string(),
                order: 0,
                default: 1.0,
                min: Some(0.0),
                max: Some(4.0),
                description: Some("Linear gain multiplier.".to_string()),
            }],
        }
    }
}

transform_sdk::export_transform!(Gain);

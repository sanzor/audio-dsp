use transform_sdk::{Direction, ParamMetadata, Params, PortCardinality, PortKind, PortMetadata, Transform, TransformMetadata};

#[derive(Default)]
pub struct HighPassFilter {
    prev_x: f32,
    prev_y: f32,
}

impl Transform for HighPassFilter {
    fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32> {
        let alpha = params[0].clamp(0.01, 0.99);
        samples[0]
            .iter()
            .map(|&x| {
                let y = alpha * (self.prev_y + x - self.prev_x);
                self.prev_x = x;
                self.prev_y = y;
                y
            })
            .collect()
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "High-Pass Filter".to_string(),
            description: Some("One-pole high-pass filter built from the previous input and output sample.".to_string()),
            ports: vec![
                PortMetadata { name: "In".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "Out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![ParamMetadata {
                name: "alpha".to_string(),
                order: 0,
                default: 0.82,
                min: Some(0.01),
                max: Some(0.99),
                description: Some("Filter coefficient. Higher values preserve more transients.".to_string()),
            }],
        }
    }
}

transform_sdk::export_transform!(HighPassFilter);

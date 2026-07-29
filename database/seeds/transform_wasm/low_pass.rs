use transform_sdk::{Direction, ParamMetadata, Params, PortCardinality, PortKind, PortMetadata, Transform, TransformMetadata};

#[derive(Default)]
pub struct LowPassFilter {
    prev_y: f32,
}

impl Transform for LowPassFilter {
    fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32> {
        let alpha = params[0].clamp(0.01, 0.99);
        samples[0]
            .iter()
            .map(|&x| {
                let y = alpha * x + (1.0 - alpha) * self.prev_y;
                self.prev_y = y;
                y
            })
            .collect()
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "Low-Pass Filter".to_string(),
            description: Some("One-pole low-pass smoothing filter.".to_string()),
            ports: vec![
                PortMetadata { name: "In".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "Out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![ParamMetadata {
                name: "alpha".to_string(),
                order: 0,
                default: 0.18,
                min: Some(0.01),
                max: Some(0.99),
                description: Some("Smoothing coefficient. Higher values pass more highs.".to_string()),
            }],
        }
    }
}

transform_sdk::export_transform!(LowPassFilter);

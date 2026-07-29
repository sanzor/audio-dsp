use transform_sdk::{Direction, ParamMetadata, Params, PortCardinality, PortKind, PortMetadata, Transform, TransformMetadata};

const DELAY_LEN: usize = 2048;

pub struct Reverb {
    delay_line: [f32; DELAY_LEN],
    delay_index: usize,
}

impl Default for Reverb {
    fn default() -> Self {
        Self { delay_line: [0.0; DELAY_LEN], delay_index: 0 }
    }
}

impl Transform for Reverb {
    fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32> {
        let mix = params[0].clamp(0.0, 1.0);
        let feedback = params[1].clamp(0.0, 0.95);

        samples[0]
            .iter()
            .map(|&dry| {
                let delayed = self.delay_line[self.delay_index];
                let wet = dry + delayed * feedback;
                self.delay_line[self.delay_index] = wet;
                self.delay_index = (self.delay_index + 1) % DELAY_LEN;
                dry * (1.0 - mix) + delayed * mix
            })
            .collect()
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "Reverb".to_string(),
            description: Some("Simple feedback delay reverb for atmospheric tails.".to_string()),
            ports: vec![
                PortMetadata { name: "In".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "Out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![
                ParamMetadata { name: "mix".to_string(), order: 0, default: 0.25, min: Some(0.0), max: Some(1.0), description: Some("Dry/wet balance.".to_string()) },
                ParamMetadata { name: "feedback".to_string(), order: 1, default: 0.45, min: Some(0.0), max: Some(0.95), description: Some("Feedback amount in the delay line.".to_string()) },
            ],
        }
    }
}

transform_sdk::export_transform!(Reverb);

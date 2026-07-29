use transform_sdk::{Direction, ParamMetadata, Params, PortCardinality, PortKind, PortMetadata, Transform, TransformMetadata};

#[derive(Default)]
pub struct Compressor;

impl Transform for Compressor {
    fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32> {
        let threshold = params[0].clamp(0.05, 1.0);
        let ratio = params[1].max(1.0);
        let makeup_gain = params[2].clamp(0.0, 4.0);

        samples[0]
            .iter()
            .map(|&sample| {
                let amplitude = sample.abs();
                let compressed = if amplitude <= threshold {
                    amplitude
                } else {
                    threshold + (amplitude - threshold) / ratio
                };
                sample.signum() * compressed * makeup_gain
            })
            .collect()
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "Compressor".to_string(),
            description: Some("Static soft-knee style amplitude compressor.".to_string()),
            ports: vec![
                PortMetadata { name: "In".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "Out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![
                ParamMetadata { name: "threshold".to_string(), order: 0, default: 0.6, min: Some(0.05), max: Some(1.0), description: Some("Absolute amplitude where compression starts.".to_string()) },
                ParamMetadata { name: "ratio".to_string(), order: 1, default: 4.0, min: Some(1.0), max: Some(20.0), description: Some("Compression ratio above threshold.".to_string()) },
                ParamMetadata { name: "makeup_gain".to_string(), order: 2, default: 1.15, min: Some(0.0), max: Some(4.0), description: Some("Linear gain applied after compression.".to_string()) },
            ],
        }
    }
}

transform_sdk::export_transform!(Compressor);

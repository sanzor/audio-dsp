use audiolib::{
    audio_buffer::AudioBuffer,
    audio_transform::{AudioTransform, AudioTransformP},
    filters::alpha,
    layout::Channels,
};
use rstest::rstest;

#[rstest]
#[case(2.0)]
pub fn can_gain(#[case] factor: f32) {
    use audiolib::layout::Channels;

    let original_gain = 1.5_f32;
    let samples: Vec<f32> = vec![original_gain];
    let sample_rate = 32.0;
    let channels = Channels::Mono;
    let buffer = AudioBuffer {
        samples: samples,
        channels: channels,
        sample_rate: sample_rate,
    };
    let x = buffer.gain(factor);
    assert_eq!(x.samples[0], (original_gain * factor));
}

#[rstest]
#[case(2.0, 24)]
pub fn can_gain_parallel(#[case] factor: f32, #[case] size: usize) {
    use audiolib::layout::Channels;

    let original_gain = 1.5_f32;
    let samples: Vec<f32> = vec![original_gain; size];
    let original_samples = samples.clone();
    let sample_rate = 32.0;
    let channels = Channels::Mono;
    let buffer = AudioBuffer {
        samples: samples,
        channels: channels,
        sample_rate: sample_rate,
    };
    let x = buffer.gain_p(factor);
    for (index, sample) in x.samples.iter().enumerate() {
        assert_eq!(*sample, (original_samples[index] * factor));
    }
}

#[rstest]
pub fn can_normalize() {
    let sample_value = 1.5_f32;
    let samples: Vec<f32> = vec![sample_value];

    let sample_rate = 32.0;
    let channels = Channels::Mono;
    let buffer = AudioBuffer {
        samples: samples,
        channels: channels,
        sample_rate: sample_rate,
    };
    let x = buffer.normalize();
    assert_eq!(x.samples[0], (sample_value * 1.0 / sample_value));
}

#[rstest]
#[case(2.0)]
pub fn can_low_pass(#[case] cutoff: f32) {
    let sample_values = vec![1.5_f32, 2.1_f32];

    let sample_rate = 32.0;
    let alpha = alpha(cutoff, sample_rate);
    let y_0 = alpha * sample_values[0];
    let y_1 = alpha * sample_values[1] + (1.0 - alpha) * y_0;

    let channels = Channels::Mono;
    let mut buffer = AudioBuffer {
        samples: sample_values,
        channels: channels,
        sample_rate: sample_rate,
    };
    buffer = buffer.low_pass(cutoff);
    assert_eq!(buffer.samples[0], y_0);
    assert_eq!(buffer.samples[1], y_1);
}

#[rstest]
#[case(2.0)]
pub fn can_high_pass(#[case] cutoff: f32) {
    let sample_values = vec![1.5_f32, 2.1_f32];
    let sample_rate = 32.0;
    let alpha = alpha(cutoff, sample_rate);
    let y_0 = alpha * sample_values[0];
    let y_1 = alpha * (y_0 + sample_values[1] - sample_values[0]);
    let channels = Channels::Mono;
    let mut buffer = AudioBuffer {
        samples: sample_values,
        channels: channels,
        sample_rate: sample_rate,
    };
    buffer = buffer.high_pass(cutoff);
    assert_eq!(buffer.samples[0], y_0);
    assert_eq!(buffer.samples[1], y_1);
}

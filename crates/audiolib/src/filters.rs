pub fn alpha(cutoff: f32, sample_rate: f32) -> f32 {
    let dt = 1.0 / sample_rate;
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
    dt / (rc + dt)
}

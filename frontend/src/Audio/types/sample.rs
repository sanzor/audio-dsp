// One block of 128 audio samples.  f32 matches the Float32Array the worklet uses.
pub struct Sample {
    pub data: [f32; 128],
}

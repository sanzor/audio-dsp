pub trait AudioTransform {
    fn gain(self, gain: f32) -> Self;
    fn normalize(self) -> Self;

    fn low_pass(self, cutoff: f32) -> Self;
    fn high_pass(self, cutoff: f32) -> Self;

    // pub fn fade_in(self,fade_length:f32)->Self;
    // pub fn fade_out(self,fade_length:f32)->Self;
    // pub fn delay(self)->Self;
}

pub trait AudioTransformMut {
    fn gain_mut(&mut self, gain: f32) -> &mut Self;
    fn normalize_mut(&mut self) -> &mut Self;

    fn low_pass_mut(&mut self, cutoff: f32) -> &mut Self;
    fn high_pass_mut(&mut self, cutoff: f32) -> &mut Self;
}
pub trait AudioTransformP {
    fn gain_p(self, gain: f32) -> Self;
    fn normalize_p(self) -> Self;
}

pub trait AudioTransformFull: AudioTransform + AudioTransformP + AudioTransformMut {}

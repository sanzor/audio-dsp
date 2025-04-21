pub trait AudioTransform{
    fn gain(&mut self,gain:f32)->&mut Self;
    // pub fn normalize(self)->Self;
    // pub fn fade_in(self,fade_length:f32)->Self;
    // pub fn fade_out(self,fade_length:f32)->Self;
    // pub fn low_pass(self,);
    // pub fn high_pass(self);
    // pub fn delay(self)->Self;
}
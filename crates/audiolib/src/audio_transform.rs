pub trait AudioTransform{
    fn gain(self,gain:f32)->Self;
    fn normalize(self)->Self;
    fn low_pass(self,cutoff:f32)->Self;
    fn high_pass(self,cutoff:f32)->Self;
    // pub fn fade_in(self,fade_length:f32)->Self;
    // pub fn fade_out(self,fade_length:f32)->Self;

    
    // pub fn delay(self)->Self;
}
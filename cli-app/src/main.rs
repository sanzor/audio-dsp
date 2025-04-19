use audiolib::audio_buffer::{ AudioBuffer,Channels};
fn main() {
    let sample_rate=32.0;
    let buffer=AudioBuffer{samples:Vec::new(),channels:Channels::Mono,sample_rate:sample_rate};
    println!("Hello, world!");
}

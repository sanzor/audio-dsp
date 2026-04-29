mod common;

#[no_mangle]
pub extern "C" fn process(input_ptr: *mut f32, len: i32, params_ptr: *const f32, params_len: i32) {
    let input = unsafe { common::input_buffer(input_ptr, len) };
    let params = unsafe { common::params(params_ptr, params_len) };
    let threshold = common::param(params, 0, 0.6).clamp(0.05, 1.0);
    let ratio = common::param(params, 1, 4.0).max(1.0);
    let makeup_gain = common::param(params, 2, 1.15).clamp(0.0, 4.0);

    for sample in input.iter_mut() {
        let amplitude = sample.abs();
        let compressed = if amplitude <= threshold {
            amplitude
        } else {
            threshold + (amplitude - threshold) / ratio
        };

        *sample = sample.signum() * compressed * makeup_gain;
    }
}

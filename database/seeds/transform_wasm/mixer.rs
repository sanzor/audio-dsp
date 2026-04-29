mod common;

#[no_mangle]
pub extern "C" fn process(input_ptr: *mut f32, len: i32, params_ptr: *const f32, params_len: i32) {
    let input = unsafe { common::input_buffer(input_ptr, len) };
    let params = unsafe { common::params(params_ptr, params_len) };
    let output_gain = common::param(params, 0, 1.0).clamp(0.0, 2.0);

    for sample in input.iter_mut() {
        *sample *= output_gain;
    }
}

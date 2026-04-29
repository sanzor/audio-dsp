mod common;

const DELAY_LEN: usize = 2048;
static mut DELAY_LINE: [f32; DELAY_LEN] = [0.0; DELAY_LEN];
static mut DELAY_INDEX: usize = 0;

#[no_mangle]
pub extern "C" fn process(input_ptr: *mut f32, len: i32, params_ptr: *const f32, params_len: i32) {
    let input = unsafe { common::input_buffer(input_ptr, len) };
    let params = unsafe { common::params(params_ptr, params_len) };
    let mix = common::param(params, 0, 0.25).clamp(0.0, 1.0);
    let feedback = common::param(params, 1, 0.45).clamp(0.0, 0.95);

    for sample in input.iter_mut() {
        let (delayed, index) = unsafe { (DELAY_LINE[DELAY_INDEX], DELAY_INDEX) };
        let dry = *sample;
        let wet = dry + delayed * feedback;

        unsafe {
            DELAY_LINE[index] = wet;
            DELAY_INDEX = (index + 1) % DELAY_LEN;
        }

        *sample = dry * (1.0 - mix) + delayed * mix;
    }
}

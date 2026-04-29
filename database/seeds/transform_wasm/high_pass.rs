mod common;

static mut PREV_X: f32 = 0.0;
static mut PREV_Y: f32 = 0.0;

#[no_mangle]
pub extern "C" fn process(input_ptr: *mut f32, len: i32, params_ptr: *const f32, params_len: i32) {
    let input = unsafe { common::input_buffer(input_ptr, len) };
    let params = unsafe { common::params(params_ptr, params_len) };
    let alpha = common::param(params, 0, 0.82).clamp(0.01, 0.99);

    for sample in input.iter_mut() {
        let x = *sample;
        let next = unsafe { alpha * (PREV_Y + x - PREV_X) };
        *sample = next;
        unsafe {
            PREV_X = x;
            PREV_Y = next;
        }
    }
}

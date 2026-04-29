use std::alloc::{alloc_zeroed, Layout};

#[no_mangle]
pub extern "C" fn alloc(len: i32) -> *mut f32 {
    let requested = len.max(1) as usize;
    let layout = Layout::array::<f32>(requested).expect("valid allocation layout");
    unsafe { alloc_zeroed(layout) as *mut f32 }
}

pub unsafe fn input_buffer<'a>(ptr: *mut f32, len: i32) -> &'a mut [f32] {
    std::slice::from_raw_parts_mut(ptr, len.max(0) as usize)
}

pub unsafe fn params<'a>(ptr: *const f32, len: i32) -> &'a [f32] {
    std::slice::from_raw_parts(ptr, len.max(0) as usize)
}

pub fn param(params: &[f32], idx: usize, default: f32) -> f32 {
    params.get(idx).copied().unwrap_or(default)
}

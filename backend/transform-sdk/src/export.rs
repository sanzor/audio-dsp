/// Generates the `extern "C"` surface the compiled wasm module must expose:
///
/// - `alloc(len) -> *mut f32` — a fixed-size bump-and-wrap arena, not a real
///   allocator. The editor worklet (`graph-worklet.js`, `callWasm`) calls
///   `alloc()` twice per audio quantum (~every 2.7ms at 48kHz/128 samples)
///   and never frees. A real allocator would grow linear memory unboundedly
///   over a playback session. Wrapping is safe here because at most two
///   pointers (input, params) are ever live at once, and each is fully
///   consumed within one synchronous `callWasm` call before the next `alloc`.
/// - `process(ptr, len, params_ptr, params_len)` — mutates the sample buffer
///   in place, matching what `callWasm` reads back after the call.
/// - `transform_metadata_ptr` / `transform_metadata_len` — expose the JSON
///   serialization of `Transform::metadata()` for the backend's post-compile
///   introspection step. Never called at audio time.
///
/// `memory` is exported automatically by `crate-type = ["cdylib"]` on
/// wasm32-unknown-unknown; nothing to do for that here.
#[macro_export]
macro_rules! export_transform {
    ($t:ty) => {
        const _TRANSFORM_SDK_ARENA_BYTES: usize = 1 << 20; // 1 MiB

        static mut _TRANSFORM_SDK_ARENA: [u8; _TRANSFORM_SDK_ARENA_BYTES] =
            [0; _TRANSFORM_SDK_ARENA_BYTES];
        static mut _TRANSFORM_SDK_ARENA_CURSOR: usize = 0;

        #[no_mangle]
        pub extern "C" fn alloc(len: usize) -> *mut f32 {
            let bytes = len * core::mem::size_of::<f32>();
            unsafe {
                if _TRANSFORM_SDK_ARENA_CURSOR + bytes > _TRANSFORM_SDK_ARENA_BYTES {
                    _TRANSFORM_SDK_ARENA_CURSOR = 0;
                }
                let ptr = _TRANSFORM_SDK_ARENA
                    .as_mut_ptr()
                    .add(_TRANSFORM_SDK_ARENA_CURSOR) as *mut f32;
                _TRANSFORM_SDK_ARENA_CURSOR += bytes;
                ptr
            }
        }

        static _TRANSFORM_SDK_INSTANCE: $crate::__private::Lazy<std::sync::Mutex<$t>> =
            $crate::__private::Lazy::new(|| std::sync::Mutex::new(<$t as Default>::default()));

        #[no_mangle]
        pub extern "C" fn process(
            ptr: *mut f32,
            len: usize,
            params_ptr: *const f32,
            params_len: usize,
        ) {
            let samples = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
            let params = unsafe { core::slice::from_raw_parts(params_ptr, params_len) };
            let mut guard = _TRANSFORM_SDK_INSTANCE.lock().unwrap();
            $crate::Transform::process(&mut *guard, samples, params);
        }

        static _TRANSFORM_SDK_METADATA_BYTES: $crate::__private::Lazy<Vec<u8>> =
            $crate::__private::Lazy::new(|| {
                $crate::__private::to_vec(&<$t as $crate::Transform>::metadata())
                    .expect("transform metadata must serialize to JSON")
            });

        #[no_mangle]
        pub extern "C" fn transform_metadata_ptr() -> *const u8 {
            _TRANSFORM_SDK_METADATA_BYTES.as_ptr()
        }

        #[no_mangle]
        pub extern "C" fn transform_metadata_len() -> usize {
            _TRANSFORM_SDK_METADATA_BYTES.len()
        }
    };
}

/// Re-exported so `export_transform!` can reference these without requiring
/// the generated user crate to depend on `once_cell`/`serde_json` directly.
#[doc(hidden)]
pub mod __private {
    pub use once_cell::sync::Lazy;
    pub use serde_json::to_vec;
}

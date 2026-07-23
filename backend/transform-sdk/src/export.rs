/// Generates the `extern "C"` surface the compiled wasm module must expose:
///
/// - `alloc(len) -> *mut f32` — a fixed-size bump-and-wrap arena, not a real
///   allocator. The editor worklet (`graph-worklet.js`, `callWasm`) calls
///   `alloc()` once per declared input port plus once for params, per audio
///   quantum (~every 2.7ms at 48kHz/128 samples), and never frees. A real
///   allocator would grow linear memory unboundedly over a playback session.
///   Wrapping is safe here because N inputs + one params buffer + one output
///   buffer are the pointers live at once per quantum (not "at most two" as
///   in the pre-multi-input-ports ABI) — still small and fixed relative to
///   the 1 MiB arena at realistic port counts — and every one of them is
///   fully consumed within one synchronous `callWasm` call before the next
///   `alloc`. A transform must not stash a pointer across calls.
/// - `process(samples_ptr, num_inputs, block_len, params_ptr, params_len) ->
///   *const f32` — reads `num_inputs` contiguous buffers of `block_len`
///   floats each (one per declared input port, in declared order) starting
///   at `samples_ptr`, calls the author's `Transform::process`, and returns
///   an arena-allocated pointer to exactly `block_len` result floats. No
///   separate length query — `callWasm` already knows `block_len`. Replaces
///   the old in-place `process(ptr, len, params_ptr, params_len)` (no return
///   value, mutated `ptr` in place) — see `transform_abi_version` below for
///   how callers tell the two apart.
/// - `transform_metadata_ptr` / `transform_metadata_len` — expose the JSON
///   serialization of `Transform::metadata()` for the backend's post-compile
///   introspection step. Never called at audio time by the backend; may be
///   evaluated lazily on the wasm side itself on the *first* `process` call,
///   since named param lookups (`Params::named`) need the declared param
///   names too — see the `_TRANSFORM_SDK_METADATA` doc comment below.
/// - `transform_abi_version() -> u32` — feature detection. Its mere presence
///   tells a caller this module speaks the new array-shaped ABI described
///   above; its absence means a module built against a pre-multi-input-ports
///   SDK, still speaking the old in-place single-buffer signature. Callers
///   (the worklet) are responsible for branching on this — not this crate's
///   concern beyond exporting it correctly.
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

        // Computed once, from whichever caller reaches it first: the
        // backend's `transform_metadata_ptr` during post-compile
        // introspection (the common case — always happens once per compile,
        // before anything is ever published), or this module's own `process`
        // on its first audio quantum if introspection never touches this
        // exact instantiation (e.g. inside the editor worklet, which never
        // calls `transform_metadata_ptr` — published ports/params come from
        // the DB, not a live wasm call). `Transform::metadata()` is
        // documented as stable for a given build, so caching it once and
        // reusing it for both the JSON export and named param lookups is
        // safe either way.
        static _TRANSFORM_SDK_METADATA: $crate::__private::Lazy<$crate::TransformMetadata> =
            $crate::__private::Lazy::new(|| <$t as $crate::Transform>::metadata());

        static _TRANSFORM_SDK_METADATA_BYTES: $crate::__private::Lazy<Vec<u8>> =
            $crate::__private::Lazy::new(|| {
                $crate::__private::to_vec(&*_TRANSFORM_SDK_METADATA)
                    .expect("transform metadata must serialize to JSON")
            });

        static _TRANSFORM_SDK_PARAM_NAMES: $crate::__private::Lazy<Vec<String>> =
            $crate::__private::Lazy::new(|| {
                _TRANSFORM_SDK_METADATA
                    .params
                    .iter()
                    .map(|p| p.name.clone())
                    .collect()
            });

        #[no_mangle]
        pub extern "C" fn process(
            samples_ptr: *const f32,
            num_inputs: usize,
            block_len: usize,
            params_ptr: *const f32,
            params_len: usize,
        ) -> *const f32 {
            let all_samples =
                unsafe { core::slice::from_raw_parts(samples_ptr, num_inputs * block_len) };
            let samples: Vec<&[f32]> = (0..num_inputs)
                .map(|i| &all_samples[i * block_len..(i + 1) * block_len])
                .collect();

            let params_values = unsafe { core::slice::from_raw_parts(params_ptr, params_len) };
            let params = $crate::Params::new(params_values, &_TRANSFORM_SDK_PARAM_NAMES);

            let result = {
                let mut guard = _TRANSFORM_SDK_INSTANCE.lock().unwrap();
                $crate::Transform::process(&mut *guard, &samples, &params)
            };

            // The ABI promises exactly `block_len` floats back regardless of
            // what the author's `process` actually returned — pad with
            // silence or truncate rather than handing the host a region
            // shorter/longer than it will read (`callWasm` always reads
            // exactly `block_len` floats from the returned pointer).
            let out_ptr = alloc(block_len);
            unsafe {
                let out = core::slice::from_raw_parts_mut(out_ptr, block_len);
                let n = result.len().min(block_len);
                out[..n].copy_from_slice(&result[..n]);
                for s in out[n..].iter_mut() {
                    *s = 0.0;
                }
            }
            out_ptr as *const f32
        }

        #[no_mangle]
        pub extern "C" fn transform_metadata_ptr() -> *const u8 {
            _TRANSFORM_SDK_METADATA_BYTES.as_ptr()
        }

        #[no_mangle]
        pub extern "C" fn transform_metadata_len() -> usize {
            _TRANSFORM_SDK_METADATA_BYTES.len()
        }

        #[no_mangle]
        pub extern "C" fn transform_abi_version() -> u32 {
            $crate::TRANSFORM_ABI_VERSION
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

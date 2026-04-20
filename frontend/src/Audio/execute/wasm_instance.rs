// Abstraction over a loaded WASM module.  The caller (worklet or main thread)
// provides the concrete implementation; the executor only calls process().
pub trait WasmInstance: Send {
    fn process(&self, input: &[f32], output: &mut [f32]);
}

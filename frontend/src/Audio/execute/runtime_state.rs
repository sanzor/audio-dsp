use crate::types::instruction::Instruction;
use super::wasm_instance::WasmInstance;

// The live state the executor holds between process() calls.
// Loaded once per graph compile; swapped atomically when the graph changes.
pub struct RuntimeState {
    pub instructions: Vec<Instruction>,
    pub instances: Vec<Box<dyn WasmInstance>>,
    // Virtual buffer pool — one slot per buf_idx in the instruction set.
    // Zeroed at the start of every block.
    pub buffers: Vec<Vec<f32>>,
    // Feedback registers — NOT zeroed between blocks.
    // Each register holds the previous block's output for one cycle-breaking edge.
    pub feedback_registry: Vec<Vec<f32>>,
    pub block_size: usize,
}

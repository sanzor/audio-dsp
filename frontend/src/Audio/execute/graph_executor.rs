use std::sync::Mutex;
use crate::types::compile_output::CompileOutput;
use crate::types::instruction::Instruction;
use super::runtime_state::RuntimeState;
use super::wasm_instance::WasmInstance;

// Runs the instruction set against the runtime state one block at a time.
// Uses a Mutex so process() can take &self — letting GraphExecutor be wrapped
// in an Arc and shared between the play path (Mixer) and the update path
// (GraphUpdater) without cloning.
pub struct GraphExecutor {
    state: Mutex<Option<RuntimeState>>,
}

impl GraphExecutor {
    pub fn new() -> Self {
        Self { state: Mutex::new(None) }
    }

    // Called by GraphUpdater when a new graph is compiled.
    // Replaces the previous RuntimeState atomically.
    pub fn load(
        &self,
        output: CompileOutput,
        instances: Vec<Box<dyn WasmInstance>>,
        block_size: usize,
    ) {
        let buffers = vec![vec![0.0f32; block_size]; output.buf_count as usize];
        let feedback_registry = vec![vec![0.0f32; block_size]; output.feedback_count as usize];
        let new_state = RuntimeState {
            instructions: output.instructions,
            instances,
            buffers,
            feedback_registry,
            block_size,
        };
        *self.state.lock().unwrap() = Some(new_state);
    }

    // Called by Mixer every audio block.
    // Walks the instruction array, routing audio through the virtual buffer pool.
    pub fn process(&self, input: &[f32], output_payload: &mut [f32]) {
        let mut guard = self.state.lock().unwrap();
        let state = match guard.as_mut() {
            Some(s) => s,
            None    => {
                output_payload.fill(0.0);
                return;
            }
        };

        for buf in &mut state.buffers {
            buf.fill(0.0);
        }

        for instr in &state.instructions {
            match instr {
                Instruction::FetchInput { target_buf_idx } => {
                    state.buffers[*target_buf_idx as usize].copy_from_slice(input);
                }
                Instruction::FetchFeedback { reg_idx, target_buf_idx } => {
                    let (tgt, regs) = (&mut state.buffers, &state.feedback_registry);
                    tgt[*target_buf_idx as usize].copy_from_slice(&regs[*reg_idx as usize]);
                }
                Instruction::Sum { source_buf_idx, target_buf_idx } => {
                    let (src_idx, tgt_idx) = (*source_buf_idx as usize, *target_buf_idx as usize);
                    // Use a split_at_mut or similar if indices might overlap, 
                    // but here they are distinct virtual buffers.
                    for i in 0..state.block_size {
                        state.buffers[tgt_idx][i] += state.buffers[src_idx][i];
                    }
                }
                Instruction::ExecuteTransform { wasm_instance_idx, input_buf_idx, output_buf_idx, .. } => {
                    let (in_idx, out_idx) = (*input_buf_idx as usize, *output_buf_idx as usize);
                    // We need to safely get two mutable references or one ref and one mut ref
                    // In a production DSP engine, you'd use unsafe or a split slice to avoid clones.
                    let (in_buf, out_buf) = unsafe {
                        let ptr = state.buffers.as_mut_ptr();
                        (&*ptr.add(in_idx), &mut *ptr.add(out_idx))
                    };
                    state.instances[*wasm_instance_idx as usize].process(in_buf, out_buf);
                }
                Instruction::StoreFeedback { source_buf_idx, reg_idx } => {
                    let (src_idx, reg_idx) = (*source_buf_idx as usize, *reg_idx as usize);
                    let src = &state.buffers[src_idx];
                    state.feedback_registry[reg_idx].copy_from_slice(src);
                }
                Instruction::WriteOutput { source_buf_idx } => {
                    output_payload.copy_from_slice(&state.buffers[*source_buf_idx as usize]);
                }
            }
        }
    }
}

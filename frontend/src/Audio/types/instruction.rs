// The instruction set — a serialisable, index-only description of how to route
// one audio block through the graph.  Produced once at compile time; read
// every block by the executor.

#[repr(u8)]
pub enum Instruction {
    FetchInput     { target_buf_idx: u32 },
    FetchFeedback  { reg_idx: u32, target_buf_idx: u32 },
    ExecuteTransform {
        wasm_instance_idx: u32,
        input_buf_idx: u32,
        output_buf_idx: u32,
        param_offset: u32,
    },
    Sum            { source_buf_idx: u32, target_buf_idx: u32 },
    StoreFeedback  { source_buf_idx: u32, reg_idx: u32 },
    WriteOutput    { source_buf_idx: u32 },
}

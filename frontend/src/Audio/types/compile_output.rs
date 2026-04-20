use super::instruction::Instruction;

// The output of compile().  Passed directly to GraphExecutor::load() so it
// knows how many virtual buffer slots and feedback registers to allocate.
// It also carries transform ids in execution order so instance loading stays
// part of the graph compilation pipeline instead of reconstructing ids later.
pub struct CompileOutput {
    pub instructions: Vec<Instruction>,
    pub buf_count: u32,
    pub feedback_count: u32,
    pub transform_ids: Vec<u32>,
}

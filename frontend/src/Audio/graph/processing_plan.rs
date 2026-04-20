use std::collections::HashMap;

// DTO produced by the graph planner and consumed by instruction building.
// This is the last graph-shaped artifact before the pipeline becomes a flat,
// buffer-indexed instruction stream.
pub struct ProcessingPlan {
    pub topo_order: Vec<i64>,
    pub node_out_buf: HashMap<i64, u32>,
    pub reg_inputs: HashMap<i64, Vec<i64>>,
    pub fb_inputs: HashMap<i64, Vec<usize>>,
    pub graph_input_buf: u32,
    pub next_buf: u32,
}

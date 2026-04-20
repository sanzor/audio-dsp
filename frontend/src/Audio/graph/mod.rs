pub mod analysis;
pub mod feedback_buffer;
pub mod feedback_edge;
pub mod instructions;
pub mod planning;
pub mod processing_plan;

use crate::types::active_graph::ActiveGraph;
use crate::types::compile_output::CompileOutput;
use crate::types::transform_store::TransformStore;

use self::analysis::{create_feedback_buffers, get_feedback_edges};
use self::instructions::create_instruction_array;
use self::planning::build_processing_plan;

pub fn compile_graph(
    graph: &ActiveGraph,
    _transform_store: &TransformStore,
) -> Result<CompileOutput, String> {
    if graph.nodes.is_empty() {
        return Err("Invalid graph: no nodes".to_string());
    }

    let feedback_edges = get_feedback_edges(graph);
    let feedback_count = feedback_edges.len() as u32;
    let feedback_buffers = create_feedback_buffers(feedback_edges);
    let plan = build_processing_plan(graph, &feedback_buffers)?;

    let transform_ids = plan
        .topo_order
        .iter()
        .map(|node_id| graph.nodes[node_id].transform_id as u32)
        .collect();

    let (instructions, buf_count) = create_instruction_array(&plan, &feedback_buffers);

    Ok(CompileOutput {
        instructions,
        buf_count,
        feedback_count,
        transform_ids,
    })
}

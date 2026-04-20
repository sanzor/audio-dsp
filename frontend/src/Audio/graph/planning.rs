use std::collections::{HashMap, HashSet, VecDeque};

use crate::types::active_graph::ActiveGraph;

use super::feedback_buffer::FeedbackBuffer;
use super::processing_plan::ProcessingPlan;

pub fn build_processing_plan(
    graph: &ActiveGraph,
    feedback_buffers: &[FeedbackBuffer],
) -> Result<ProcessingPlan, String> {
    let feedback_set: HashSet<(i64, i64)> = feedback_buffers
        .iter()
        .map(|buffer| (buffer.from_id, buffer.to_id))
        .collect();

    let mut in_degree: HashMap<i64, usize> = graph.nodes.keys().map(|&id| (id, 0)).collect();
    let mut adjacency: HashMap<i64, Vec<i64>> =
        graph.nodes.keys().map(|&id| (id, vec![])).collect();

    for edge in graph.edges.values() {
        if !feedback_set.contains(&(edge.from_node_id, edge.to_node_id)) {
            adjacency
                .get_mut(&edge.from_node_id)
                .unwrap()
                .push(edge.to_node_id);
            *in_degree.get_mut(&edge.to_node_id).unwrap() += 1;
        }
    }

    let mut queue: VecDeque<i64> = in_degree
        .iter()
        .filter(|(_, &degree)| degree == 0)
        .map(|(&id, _)| id)
        .collect();

    let mut topo_order: Vec<i64> = Vec::with_capacity(graph.nodes.len());

    while let Some(node_id) = queue.pop_front() {
        topo_order.push(node_id);

        for &neighbor in &adjacency[&node_id] {
            let degree = in_degree.get_mut(&neighbor).unwrap();
            *degree -= 1;

            if *degree == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    if topo_order.len() != graph.nodes.len() {
        return Err("Cycle remains after removing feedback edges".to_string());
    }

    let mut reg_inputs: HashMap<i64, Vec<i64>> =
        graph.nodes.keys().map(|&id| (id, vec![])).collect();
    let mut fb_inputs: HashMap<i64, Vec<usize>> =
        graph.nodes.keys().map(|&id| (id, vec![])).collect();

    for edge in graph.edges.values() {
        let key = (edge.from_node_id, edge.to_node_id);

        if feedback_set.contains(&key) {
            if let Some(reg_idx) = feedback_buffers.iter().position(|buffer| {
                buffer.from_id == edge.from_node_id && buffer.to_id == edge.to_node_id
            }) {
                fb_inputs.get_mut(&edge.to_node_id).unwrap().push(reg_idx);
            }
        } else {
            reg_inputs
                .get_mut(&edge.to_node_id)
                .unwrap()
                .push(edge.from_node_id);
        }
    }

    let graph_input_buf: u32 = 0;
    let node_out_buf: HashMap<i64, u32> = topo_order
        .iter()
        .enumerate()
        .map(|(index, &id)| (id, 1 + index as u32))
        .collect();
    let next_buf: u32 = 1 + topo_order.len() as u32;

    Ok(ProcessingPlan {
        topo_order,
        node_out_buf,
        reg_inputs,
        fb_inputs,
        graph_input_buf,
        next_buf,
    })
}

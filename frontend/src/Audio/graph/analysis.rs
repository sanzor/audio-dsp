use std::collections::HashSet;

use crate::types::active_graph::ActiveGraph;

use super::feedback_buffer::FeedbackBuffer;
use super::feedback_edge::FeedbackEdge;

pub fn get_feedback_edges(graph: &ActiveGraph) -> Vec<FeedbackEdge> {
    let mut feedback_edges: Vec<FeedbackEdge> = Vec::new();
    let mut visited: HashSet<i64> = HashSet::new();
    let mut visiting: HashSet<i64> = HashSet::new();

    for &node_id in graph.nodes.keys() {
        if !visited.contains(&node_id) {
            dfs(
                node_id,
                graph,
                &mut visited,
                &mut visiting,
                &mut feedback_edges,
            );
        }
    }

    feedback_edges
}

pub fn create_feedback_buffers(edges: Vec<FeedbackEdge>) -> Vec<FeedbackBuffer> {
    edges
        .into_iter()
        .map(|edge| FeedbackBuffer {
            from_id: edge.from_id,
            to_id: edge.to_id,
            size: edge.size,
        })
        .collect()
}

fn dfs(
    node_id: i64,
    graph: &ActiveGraph,
    visited: &mut HashSet<i64>,
    visiting: &mut HashSet<i64>,
    feedback_edges: &mut Vec<FeedbackEdge>,
) {
    visiting.insert(node_id);

    let neighbors: Vec<i64> = graph
        .edges
        .values()
        .filter(|edge| edge.from_node_id == node_id)
        .map(|edge| edge.to_node_id)
        .collect();

    for target in neighbors {
        if visiting.contains(&target) {
            feedback_edges.push(FeedbackEdge {
                from_id: node_id,
                to_id: target,
                size: 128,
            });
        } else if !visited.contains(&target) {
            dfs(target, graph, visited, visiting, feedback_edges);
        }
    }

    visiting.remove(&node_id);
    visited.insert(node_id);
}

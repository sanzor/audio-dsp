use std::collections::HashMap;

pub enum BinaryStatus { Idle, Loading, Ready, Error }

pub struct Position { pub x: f64, pub y: f64 }

pub struct ActiveNode {
    pub id: i64,
    pub transform_id: i64,
    pub position: Position,
    pub params: HashMap<String, f64>,
    pub binary: Option<Vec<u8>>,
    pub binary_status: BinaryStatus,
    pub binary_error: Option<String>,
}

pub struct ActiveEdge {
    pub id: i64,
    pub from_node_id: i64,
    pub to_node_id: i64,
    pub from_port_id: i64,
    pub to_port_id: i64,
}

pub struct ActiveGraph {
    pub id: Option<i64>,
    pub region_id: Option<i64>,
    pub name: String,
    pub nodes: HashMap<i64, ActiveNode>,
    pub edges: HashMap<i64, ActiveEdge>,
    pub is_dirty: bool,
    pub enabled: bool,
}

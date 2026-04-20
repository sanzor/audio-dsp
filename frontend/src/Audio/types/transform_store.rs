use std::collections::HashMap;

pub struct Transform {
    pub name: String,
    pub id: String,
    pub description: String,
    pub compiled_binary: Vec<u8>,
}

pub struct TransformStore {
    pub store: HashMap<String, Transform>,
}

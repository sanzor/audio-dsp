use crate::db::GraphId;

pub struct EditGraphParams {
    pub graph_id: GraphId,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct RegionDbDto {
    pub id: String,
    pub region_set_id: String,
    pub name: String,
    pub start: f64,
    pub end: f64,
    pub graph_id: Option<String>,
}

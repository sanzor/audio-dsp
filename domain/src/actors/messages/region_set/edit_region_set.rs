use dtos::db::region_set_db_dto::RegionSetDbDto;

pub struct EditRegionSet {
    pub region_set_id: String,
    pub name: Option<String>,
}

pub struct EditRegionSetResult {
    pub region_set: RegionSetDbDto,
}

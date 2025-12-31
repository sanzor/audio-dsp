use dtos::db::region_set_db_dto::RegionSetDbDto;

pub struct CreateRegionSet {
    pub track_id: String,
    pub name: Option<String>,
}

pub struct CreateRegionSetResult {
    pub region_set: RegionSetDbDto,
}

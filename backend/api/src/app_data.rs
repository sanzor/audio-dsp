use std::{marker::PhantomData, sync::Arc};

use crate::{
    graphs::graphs_provider::GraphsProvider,
    memberships::memberships_provider::MembershipsProvider,
    player::player_provider::PlayerProvider,
    projects::projects_provider::ProjectsProvider,
    region_sets::region_sets_provider::RegionSetsProvider,
    regions::regions_provider::RegionsProvider,
    tracks::tracks_provider::TracksProvider,
    users::user_provider::UserProvider,
};

#[derive(Clone)]
pub struct AppData {
    pub player_service: Arc<dyn PlayerProvider>,
    pub tracks_service: Arc<dyn TracksProvider>,
    pub region_sets_service: Arc<dyn RegionSetsProvider>,
    pub regions_service: Arc<dyn RegionsProvider>,
    pub graphs_service: Arc<dyn GraphsProvider>,
    pub projects_service: Arc<dyn ProjectsProvider>,
    pub memberships_service: Arc<dyn MembershipsProvider>,
    pub user_provider: Arc<dyn UserProvider>,
}

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Audio DSP API", version = "0.1.0"),
    paths(
        crate::controllers::google_controller::google_auth_redirect,
        crate::controllers::google_controller::google_callback,
        crate::controllers::google_controller::logout,
        crate::controllers::google_controller::refresh,
        crate::controllers::google_controller::session,
        crate::controllers::player_controller::get_player_state,
        crate::controllers::player_controller::play,
        crate::controllers::player_controller::pause,
        crate::controllers::player_controller::seek,
        crate::controllers::player_controller::stop,
        crate::controllers::tracks_crud_controller::add_track,
        crate::controllers::tracks_crud_controller::add_track_multi,
        crate::controllers::tracks_crud_controller::copy_track,
        crate::controllers::tracks_crud_controller::update_track_info,
        crate::controllers::tracks_crud_controller::remove_track,
        crate::controllers::tracks_crud_controller::get_stored_track,
        crate::controllers::tracks_crud_controller::get_meta,
        crate::controllers::tracks_crud_controller::get_tracks,
        crate::controllers::tracks_crud_controller::get_track_info,
        crate::controllers::regions_controller::add_region,
        crate::controllers::regions_controller::edit_region,
        crate::controllers::regions_controller::remove_region,
        crate::controllers::regions_controller::copy_region,
        crate::controllers::region_set_controller::create_region_set,
        crate::controllers::region_set_controller::edit_region_set,
        crate::controllers::region_set_controller::get_region_set,
        crate::controllers::region_set_controller::get_region_sets,
        crate::controllers::region_set_controller::get_region_sets_for_track,
        crate::controllers::region_set_controller::delete_region_set,
        crate::controllers::region_set_controller::copy_region_set,
        crate::controllers::user_controller::get_user_state,
        crate::controllers::user_controller::delete,
        crate::controllers::user_controller::update,
        crate::controllers::ws_controller::run_player
    ),
    tags(
        (name = "auth", description = "Authentication (Google OAuth)"),
        (name = "player", description = "Player commands"),
        (name = "tracks", description = "Tracks CRUD"),
        (name = "regions", description = "Regions CRUD"),
        (name = "region-sets", description = "Region sets CRUD"),
        (name = "user", description = "User operations"),
        (name = "ws", description = "Websocket endpoints")
    )
)]
pub struct ApiDoc;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Audio DSP API", version = "0.1.0"),
    paths(
        // Auth (email/password)
        crate::controllers::auth_controller::login,
        crate::controllers::auth_controller::register,
        crate::controllers::auth_controller::verify,
        crate::controllers::auth_controller::resend_verification,
        crate::controllers::auth_controller::logout,
        // Me
        crate::controllers::me_controller::bootstrap,
        crate::controllers::me_controller::create_project,
        crate::controllers::me_controller::accept_invite,
        // Projects
        crate::controllers::project_controller::list_members,
        crate::controllers::project_controller::invite_member,
        crate::controllers::project_controller::delete_project,
        crate::controllers::project_controller::remove_member,
        crate::controllers::project_controller::change_role,
        // Player
        crate::controllers::player_controller::get_player_state,
        crate::controllers::player_controller::play,
        crate::controllers::player_controller::pause,
        crate::controllers::player_controller::seek,
        crate::controllers::player_controller::stop,
        // Tracks
        crate::controllers::tracks_crud_controller::add_track,
        crate::controllers::tracks_crud_controller::add_track_multi,
        crate::controllers::tracks_crud_controller::copy_track,
        crate::controllers::tracks_crud_controller::update_track_info,
        crate::controllers::tracks_crud_controller::remove_track,
        crate::controllers::tracks_crud_controller::get_meta,
        crate::controllers::track_payload_controller::get_stored_track,
        crate::controllers::tracks_crud_controller::get_tracks,
        crate::controllers::tracks_crud_controller::get_track_info,
        // Transforms
        crate::controllers::transforms_controller::list_transform_summaries,
        crate::controllers::transforms_controller::get_transform_definition,
        crate::controllers::transforms_controller::resolve_transform_definitions,
        crate::controllers::transforms_controller::get_transform_binary,
        crate::controllers::transforms_controller::get_transform_binaries,
        crate::controllers::transforms_controller::create_transform,
        crate::controllers::transforms_controller::update_transform,
        crate::controllers::transforms_controller::delete_transform,
        crate::controllers::transforms_controller::add_port,
        crate::controllers::transforms_controller::delete_port,
        // Tickets
        crate::controllers::ticket_controller::create_compile_ticket,
        crate::controllers::ticket_controller::get_compile_ticket_status,
        crate::controllers::ticket_controller::get_compile_resource,
        // Regions
        crate::controllers::regions_controller::add_region,
        crate::controllers::regions_controller::edit_region,
        crate::controllers::regions_controller::remove_region,
        crate::controllers::regions_controller::copy_region,
        // Region sets
        crate::controllers::region_set_controller::create_region_set,
        crate::controllers::region_set_controller::edit_region_set,
        crate::controllers::region_set_controller::get_region_set,
        crate::controllers::region_set_controller::get_region_sets,
        crate::controllers::region_set_controller::get_region_sets_for_track,
        crate::controllers::region_set_controller::delete_region_set,
        crate::controllers::region_set_controller::copy_region_set,
        // Users
        crate::controllers::user_controller::create_user,
        crate::controllers::user_controller::update_user,
        crate::controllers::user_controller::delete_user,
        crate::controllers::user_controller::get_user,
        crate::controllers::user_controller::get_all_users,
        // WebSocket
        crate::controllers::ws_controller::run_player
    ),
    tags(
        (name = "Auth", description = "Email/password authentication"),
        (name = "Me", description = "Current user bootstrap & project creation"),
        (name = "Projects", description = "Project & membership management"),
        (name = "player", description = "Player commands"),
        (name = "tracks", description = "Tracks CRUD"),
        (name = "transforms", description = "Transforms CRUD and WASM retrieval"),
        (name = "tickets", description = "Compile ticket lifecycle and resource retrieval"),
        (name = "regions", description = "Regions CRUD"),
        (name = "region-sets", description = "Region sets CRUD"),
        (name = "user", description = "User operations"),
        (name = "ws", description = "Websocket endpoints"),
        (name = "stored-tracks", description = "Stored track audio retrieval")
    )
)]
pub struct ApiDoc;

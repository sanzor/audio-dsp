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
        crate::controllers::me_controller::create_workspace,
        crate::controllers::me_controller::accept_invite,
        // Projects
        crate::controllers::workspace_controller::list_members,
        crate::controllers::workspace_controller::invite_member,
        crate::controllers::workspace_controller::delete_workspace,
        crate::controllers::workspace_controller::remove_member,
        crate::controllers::workspace_controller::change_role,
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
        // Sources
        crate::controllers::sources_controller::add_source,
        crate::controllers::sources_controller::add_source_multi,
        crate::controllers::sources_controller::list_sources,
        crate::controllers::sources_controller::rename_source,
        crate::controllers::sources_controller::delete_source,
        crate::controllers::sources_controller::get_source_audio,
        // Transforms (published, bucket 3)
        crate::controllers::transforms_controller::list_transform_summaries,
        crate::controllers::transforms_controller::get_transform,
        crate::controllers::transforms_controller::get_transforms,
        crate::controllers::transforms_controller::get_transform_binaries,
        crate::controllers::transforms_controller::delete_transform,
        // Transform drafts (bucket 2 + the actions that act on it)
        crate::controllers::transform_drafts_controller::create_transform_draft,
        crate::controllers::transform_drafts_controller::get_transform_draft,
        crate::controllers::transform_drafts_controller::get_transform_drafts,
        crate::controllers::transform_drafts_controller::save_draft,
        crate::controllers::transform_drafts_controller::validate_source,
        crate::controllers::transform_drafts_controller::validate_graph_draft,
        crate::controllers::transform_drafts_controller::publish_primitive,
        crate::controllers::transform_drafts_controller::publish_composite,
        crate::controllers::transform_drafts_controller::delete_transform_draft,
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
        (name = "Workspaces", description = "Workspace & membership management"),
        (name = "player", description = "Player commands"),
        (name = "tracks", description = "Tracks CRUD"),
        (name = "sources", description = "Creator-surface signal sources CRUD and audition/preview audio retrieval"),
        (name = "transforms", description = "Published transforms: catalog reads, WASM binary retrieval, deletion"),
        (name = "draft_transforms", description = "Transform draft lifecycle: create/save/read/delete, plus validate and publish"),
        (name = "tickets", description = "Compile ticket lifecycle and resource retrieval"),
        (name = "regions", description = "Regions CRUD"),
        (name = "region-sets", description = "Region sets CRUD"),
        (name = "user", description = "User operations"),
        (name = "ws", description = "Websocket endpoints"),
        (name = "stored-tracks", description = "Stored track audio retrieval")
    )
)]
pub struct ApiDoc;

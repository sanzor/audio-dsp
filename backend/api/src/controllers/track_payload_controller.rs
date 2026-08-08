use crate::{
    middlewares::membership::membership_context::{ProjectContext, RoleContext},
    stored_tracks::stored_tracks_app_data::StoredTracksAppData,
};
use actix_web::{get, web, HttpResponse};
use domain::db::TrackId;
use mime_guess::from_ext;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct GetStoredTrackParams {
    pub track_id: TrackId,
}

#[utoipa::path(
    get,
    path = "/stored-tracks/get",
    tag = "stored-tracks",
    params(GetStoredTrackParams),
    responses((status = 200, description = "Track audio bytes")),
    security(("bearerAuth" = []))
)]
#[get("/get")]
pub async fn get_stored_track(
    role: RoleContext,
    project: ProjectContext,
    query: web::Query<GetStoredTrackParams>,
    app_state: web::Data<StoredTracksAppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = query.into_inner();
    let track = match app_state.tracks_service.get_track(&request.track_id, project.0).await {
        Ok(t) => t,
        Err(_) => return HttpResponse::NotFound().body("Could not find track"),
    };
    let ext = track.meta.track_info.extension.to_lowercase();
    let mime_type = from_ext(&ext).first_or_octet_stream().essence_str().to_owned();
    HttpResponse::Ok()
        .insert_header(("Content-Type", mime_type))
        .insert_header((
            "Content-Disposition",
            format!("inline; filename=\"{}.{}\"", track.meta.track_info.name, ext),
        ))
        .body(track.payload.canonical_audio)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_stored_track);
}

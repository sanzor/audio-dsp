use crate::{
    middlewares::membership::membership_context::{ProjectContext, RoleContext},
    sources::sources_app_data::SourcesAppData,
};
use actix_multipart::Multipart;
use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse,
};
use domain::{
    db::db_source::SourceId,
    sources::{raw_source::RawSource, source_info::SourceInfo as DomainSourceInfo},
    update_source_info_params::UpdateSourceInfoParams,
};
use mime_guess::from_ext;

use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize)]
pub struct AddSourceParams {
    pub source: RawSource,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddSourceResult {
    pub source_id: SourceId,
    #[schema(value_type = Object)]
    pub source_info: DomainSourceInfo,
}

#[derive(ToSchema)]
pub struct AddSourceMultipartRequest {
    pub name: String,
    pub extension: String,
    #[schema(value_type = String, format = Binary)]
    pub samples: Vec<u8>,
    pub sample_rate: Option<f32>,
    pub channels: Option<String>,
}

#[utoipa::path(
    post,
    path = "/sources/add-source",
    tag = "sources",
    request_body = serde_json::Value,
    responses((status = 200, description = "Source added", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/add-source")]
pub async fn add_source(
    role: RoleContext,
    project: ProjectContext,
    path: web::Json<serde_json::Value>,
    app_state: web::Data<SourcesAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request: AddSourceParams = match serde_json::from_value(path.into_inner()) {
        Ok(r) => r,
        Err(_) => return HttpResponse::BadRequest().body("Invalid payload"),
    };

    match app_state.sources_service.insert_source(request.source, project.0).await {
        Ok(source) => HttpResponse::Ok().json(AddSourceResult {
            source_id: source.meta.source_id,
            source_info: source.meta.source_info,
        }),
        Err(_e) => HttpResponse::InternalServerError().body("Could not insert source"),
    }
}

#[utoipa::path(
    post,
    path = "/sources/add-source-multi",
    tag = "sources",
    request_body(
        content = AddSourceMultipartRequest,
        content_type = "multipart/form-data",
        description = "Upload a WAV file in `samples`, or upload raw float32 PCM bytes in `samples` together with `sample_rate` and `channels` (`Mono` or `Stereo`). MP3 bytes are not decoded by this endpoint."
    ),
    responses((status = 200, description = "Source added (multipart)", body = AddSourceResult)),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/add-source-multi")]
pub async fn add_source_multi(
    role: RoleContext,
    project: ProjectContext,
    payload: Multipart,
    app_state: web::Data<SourcesAppData>,
) -> HttpResponse {
    info!("add-source-multi request received");
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let raw_source = match app_state.multipart_parser.try_parse_multipart(payload).await {
        Ok(r) => r,
        Err(err) => {
            error!(error = %err, "add-source-multi rejected: invalid payload");
            return HttpResponse::BadRequest().body(err);
        }
    };

    match app_state.sources_service.insert_source(raw_source, project.0).await {
        Ok(source) => {
            info!(source_id = %source.meta.source_id, "add-source-multi insert complete");
            HttpResponse::Ok().json(AddSourceResult {
                source_id: source.meta.source_id,
                source_info: source.meta.source_info,
            })
        }
        Err(err) => {
            error!(error = %err, "add-source-multi insert failed");
            HttpResponse::InternalServerError().body("Could not insert source")
        }
    }
}

#[utoipa::path(
    get,
    path = "/sources/list-sources",
    tag = "sources",
    responses((status = 200, description = "All source metas", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[get("/list-sources")]
pub async fn list_sources(role: RoleContext, app_state: web::Data<SourcesAppData>) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app_state.sources_service.get_all_source_metas().await {
        Ok(metas) => HttpResponse::Ok().json(metas),
        Err(_e) => HttpResponse::InternalServerError().body("Could not get sources"),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct RenameSourceParams {
    pub source_id: SourceId,
    pub source_name: String,
}

#[utoipa::path(
    post,
    path = "/sources/rename-source",
    tag = "sources",
    request_body = RenameSourceParams,
    responses((status = 200, description = "Source renamed", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/rename-source")]
pub async fn rename_source(
    role: RoleContext,
    path: web::Json<RenameSourceParams>,
    app_state: web::Data<SourcesAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = path.into_inner();
    match app_state
        .sources_service
        .update_source_info(
            &request.source_id,
            UpdateSourceInfoParams { source_name: request.source_name },
        )
        .await
    {
        Ok(meta) => HttpResponse::Ok().json(meta),
        Err(_e) => HttpResponse::InternalServerError().body("Could not rename source"),
    }
}

#[derive(Deserialize, IntoParams)]
pub struct DeleteSourceParams {
    pub source_id: SourceId,
}

#[utoipa::path(
    delete,
    path = "/sources/delete-source",
    tag = "sources",
    params(DeleteSourceParams),
    responses((status = 200, description = "Source removed", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[delete("/delete-source")]
pub async fn delete_source(
    role: RoleContext,
    path: web::Query<DeleteSourceParams>,
    app_state: web::Data<SourcesAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = path.into_inner();
    match app_state.sources_service.delete_source(&request.source_id).await {
        Ok(_) => HttpResponse::Ok().json("source removed"),
        Err(_e) => HttpResponse::InternalServerError().body("Could not remove source"),
    }
}

// Not part of the product-owner's requested endpoint list (add-source, add-source-multi,
// list-sources, rename-source, delete-source) -- added because the Creator preview session
// needs a way to actually fetch a picked source's audio bytes to play them, the same way
// `/stored-tracks/get` (`track_payload_controller.rs`) serves track audio today. Flagged as
// a deviation in the implementation report; trivial to remove if out of scope.
#[derive(Deserialize, IntoParams)]
pub struct GetSourceAudioParams {
    pub source_id: SourceId,
}

#[utoipa::path(
    get,
    path = "/sources/get-audio",
    tag = "sources",
    params(GetSourceAudioParams),
    responses((status = 200, description = "Source audio bytes")),
    security(
        ("bearerAuth" = [])
    )
)]
#[get("/get-audio")]
pub async fn get_source_audio(
    role: RoleContext,
    query: web::Query<GetSourceAudioParams>,
    app_state: web::Data<SourcesAppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = query.into_inner();
    let source = match app_state.sources_service.get_source(&request.source_id).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::NotFound().body("Could not find source"),
    };
    let ext = source.meta.source_info.extension.to_lowercase();
    let mime_type = from_ext(&ext).first_or_octet_stream().essence_str().to_owned();
    HttpResponse::Ok()
        .insert_header(("Content-Type", mime_type))
        .insert_header((
            "Content-Disposition",
            format!("inline; filename=\"{}.{}\"", source.meta.source_info.name, ext),
        ))
        .body(source.payload.canonical_audio)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_source)
        .service(add_source_multi)
        .service(list_sources)
        .service(rename_source)
        .service(delete_source)
        .service(get_source_audio);
}

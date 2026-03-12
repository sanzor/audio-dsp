use actix_web::{delete, get, patch, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use domain::{
    db::{DbRegionSet, RegionSetId, TrackId},
    region_set::region_set_subtree::RegionSetSubtree,
    regions::region_subtree::RegionSubtree,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

use crate::{
    app_data::AppData,
    middlewares::role_context::role_context::RoleContext,
    region_sets::region_sets_provider::{
        CopyRegionSetParams, CreateRegionSetParams, EditRegionSetParams,
    },
};

// ── create ────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateRegionSetRequest {
    #[serde(rename = "trackId")]
    pub track_id: TrackId,
    pub name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegionSetResult {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: String,
    pub track_length_seconds: f32,
    pub created_at: DateTime<Utc>,
}

impl From<DbRegionSet> for CreateRegionSetResult {
    fn from(db: DbRegionSet) -> Self {
        CreateRegionSetResult {
            region_set_id: db.region_set_id,
            track_id: db.track_id,
            name: db.name,
            track_length_seconds: db.track_length_seconds,
            created_at: db.created_at,
        }
    }
}

#[utoipa::path(
    post,
    path = "/region-sets/create",
    tag = "region-sets",
    request_body = CreateRegionSetRequest,
    responses(
        (status = 201, description = "Region Set Created", body = serde_json::Value),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/create")]
pub async fn create_region_set(
    role: RoleContext,
    request: web::Json<CreateRegionSetRequest>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state
        .region_sets_service
        .create_region_set(CreateRegionSetParams {
            track_id: request.track_id,
            name: request.name,
        })
        .await
    {
        Ok(r) => HttpResponse::Created().json(CreateRegionSetResult::from(r)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not create region set"),
    }
}

// ── edit ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EditRegionSetRequest {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRegionSetResult {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: String,
    pub track_length_seconds: f32,
    pub created_at: DateTime<Utc>,
}

impl From<DbRegionSet> for EditRegionSetResult {
    fn from(db: DbRegionSet) -> Self {
        EditRegionSetResult {
            region_set_id: db.region_set_id,
            track_id: db.track_id,
            name: db.name,
            track_length_seconds: db.track_length_seconds,
            created_at: db.created_at,
        }
    }
}

#[utoipa::path(
    patch,
    path = "/region-sets/edit",
    tag = "region-sets",
    request_body = EditRegionSetRequest,
    responses((status = 200, description = "Region set edited", body = serde_json::Value))
)]
#[patch("/edit")]
pub async fn edit_region_set(
    role: RoleContext,
    request: web::Json<EditRegionSetRequest>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state
        .region_sets_service
        .edit_region_set(EditRegionSetParams {
            name: request.name,
            region_set_id: request.region_set_id,
            track_id: request.track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(EditRegionSetResult::from(r)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not edit region set"),
    }
}

// ── get ───────────────────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
pub struct GetRegionSetParams {
    pub region_set_id: RegionSetId,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRegionSetResult {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: String,
    pub track_length_seconds: f32,
    pub created_at: DateTime<Utc>,
}

impl From<DbRegionSet> for GetRegionSetResult {
    fn from(db: DbRegionSet) -> Self {
        GetRegionSetResult {
            region_set_id: db.region_set_id,
            track_id: db.track_id,
            name: db.name,
            track_length_seconds: db.track_length_seconds,
            created_at: db.created_at,
        }
    }
}

#[utoipa::path(
    get,
    path = "/region-sets/get",
    tag = "region-sets",
    params(GetRegionSetParams),
    responses((status = 200, description = "Region set", body = serde_json::Value))
)]
#[get("/get")]
pub async fn get_region_set(
    role: RoleContext,
    request: web::Query<GetRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state.region_sets_service.get_region_set(&request.region_set_id).await {
        Ok(r) => HttpResponse::Ok().json(GetRegionSetResult::from(r)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not find region set"),
    }
}

// ── get-all ───────────────────────────────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRegionSetsResult {
    pub track_region_sets_map: HashMap<TrackId, Vec<GetRegionSetResult>>,
}

#[utoipa::path(
    get,
    path = "/region-sets/get-all",
    tag = "region-sets",
    responses((status = 200, description = "All region sets", body = serde_json::Value))
)]
#[get("/get-all")]
pub async fn get_region_sets(
    role: RoleContext,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app_state.region_sets_service.get_region_sets().await {
        Ok(map) => HttpResponse::Ok().json(GetRegionSetsResult {
            track_region_sets_map: map
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(GetRegionSetResult::from).collect()))
                .collect(),
        }),
        Err(_e) => HttpResponse::InternalServerError().body("Could not get region sets"),
    }
}

// ── get-all-for-track ─────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
pub struct GetRegionsForTrackParams {
    #[serde(rename = "trackId")]
    pub track_id: TrackId,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRegionSetsForTrackResult {
    pub track_id: TrackId,
    pub region_sets: Vec<GetRegionSetResult>,
}

#[utoipa::path(
    get,
    path = "/region-sets/get-all-for-track",
    tag = "region-sets",
    params(GetRegionsForTrackParams),
    responses((status = 200, description = "Region sets for a track", body = serde_json::Value))
)]
#[get("/get-all-for-track")]
pub async fn get_region_sets_for_track(
    role: RoleContext,
    request: web::Query<GetRegionsForTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state.region_sets_service.get_region_sets_for_track(&request.track_id).await {
        Ok(sets) => HttpResponse::Ok().json(GetRegionSetsForTrackResult {
            track_id: request.track_id,
            region_sets: sets.into_iter().map(GetRegionSetResult::from).collect(),
        }),
        Err(_e) => HttpResponse::InternalServerError().body("Could not get region sets for track"),
    }
}

// ── delete ────────────────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
pub struct DeleteRegionSetParams {
    #[serde(rename = "regionSetId")]
    pub region_set_id: RegionSetId,
}

#[utoipa::path(
    delete,
    path = "/region-sets/delete",
    tag = "region-sets",
    params(DeleteRegionSetParams),
    responses((status = 200, description = "Region set deleted"))
)]
#[delete("/delete")]
pub async fn delete_region_set(
    role: RoleContext,
    request: web::Query<DeleteRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.is_owner() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state.region_sets_service.delete_region_set(&request.region_set_id).await {
        Ok(_) => HttpResponse::Ok().body("Region set deleted"),
        Err(_e) => HttpResponse::InternalServerError().body("Could not delete region set"),
    }
}

// ── copy ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CopyRegionSetRequest {
    #[serde(rename = "regionSetId")]
    pub region_set_id: RegionSetId,
    #[serde(rename = "copyName")]
    pub copy_region_set_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyRegionSetResult {
    pub track_id: TrackId,
    pub track_length: f32,
    pub region_set_id: RegionSetId,
    pub name: String,
    pub regions: Vec<RegionSubtree>,
}

impl From<RegionSetSubtree> for CopyRegionSetResult {
    fn from(s: RegionSetSubtree) -> Self {
        CopyRegionSetResult {
            track_id: s.track_id,
            track_length: s.track_length,
            region_set_id: s.region_set_id,
            name: s.name,
            regions: s.regions,
        }
    }
}

#[utoipa::path(
    post,
    path = "/region-sets/copy-region-set",
    tag = "region-sets",
    request_body = CopyRegionSetRequest,
    responses((status = 200, description = "Region set copied", body = serde_json::Value))
)]
#[post("/copy-region-set")]
pub async fn copy_region_set(
    role: RoleContext,
    request_raw: web::Json<CopyRegionSetRequest>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request_raw.into_inner();
    match app_state
        .region_sets_service
        .copy_region_set(CopyRegionSetParams {
            region_set_id: request.region_set_id,
            copy_name: request.copy_region_set_name,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(CopyRegionSetResult::from(r)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not copy region set"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_region_set)
        .service(delete_region_set)
        .service(edit_region_set)
        .service(get_region_set)
        .service(get_region_sets)
        .service(get_region_sets_for_track)
        .service(copy_region_set);
}

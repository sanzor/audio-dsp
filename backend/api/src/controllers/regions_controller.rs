use actix_web::{delete, patch, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use domain::db::{DbRegionSet, RegionId, RegionSetId, TrackId};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    middlewares::role_context::role_context::RoleContext,
    regions::{
        regions_app_data::RegionsAppData,
        regions_provider::{AddRegionParams, CopyRegionParams, DeleteRegionParams, EditRegionParams, EndTimePolicy},
    },
};

// Shared region-set result fields (1:1 with DbRegionSet)
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionSetResult {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: String,
    pub track_length_seconds: f32,
    pub created_at: DateTime<Utc>,
}

impl From<DbRegionSet> for RegionSetResult {
    fn from(db: DbRegionSet) -> Self {
        RegionSetResult {
            region_set_id: db.region_set_id,
            track_id: db.track_id,
            name: db.name,
            track_length_seconds: db.track_length_seconds,
            created_at: db.created_at,
        }
    }
}

// ── add ───────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddRegionRequest {
    pub region_set_id: RegionSetId,
    pub start_time: f32,
    pub name: String,
    pub end_time_policy: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRegionResult {
    pub region_set: RegionSetResult,
}

impl From<DbRegionSet> for AddRegionResult {
    fn from(db: DbRegionSet) -> Self {
        AddRegionResult {
            region_set: RegionSetResult::from(db),
        }
    }
}

#[utoipa::path(
    post,
    path = "/regions/add",
    tag = "regions",
    request_body = AddRegionRequest,
    responses((status = 201, description = "Region added", body = serde_json::Value))
)]
#[post("/add")]
pub async fn add_region(
    role: RoleContext,
    request: web::Json<AddRegionRequest>,
    app_state: web::Data<RegionsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state
        .regions_service
        .add_region(AddRegionParams {
            name: request.name,
            region_set_id: request.region_set_id,
            start_time: request.start_time,
            end_time_policy: EndTimePolicy::NextRegionOrEnd,
        })
        .await
    {
        Ok(r) => HttpResponse::Created().json(AddRegionResult::from(r)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not add region"),
    }
}

// ── edit ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EditRegionRequest {
    pub region_id: RegionId,
    pub region_set_id: RegionSetId,
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
    pub name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRegionResult {
    pub region_set: RegionSetResult,
}

impl From<DbRegionSet> for EditRegionResult {
    fn from(db: DbRegionSet) -> Self {
        EditRegionResult {
            region_set: RegionSetResult::from(db),
        }
    }
}

#[utoipa::path(
    patch,
    path = "/regions/edit",
    tag = "regions",
    request_body = EditRegionRequest,
    responses((status = 200, description = "Region edited", body = serde_json::Value))
)]
#[patch("/edit")]
pub async fn edit_region(
    role: RoleContext,
    request: web::Json<EditRegionRequest>,
    app_state: web::Data<RegionsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state
        .regions_service
        .edit_region(EditRegionParams {
            name: request.name,
            region_set_id: request.region_set_id,
            region_id: request.region_id,
            start_time: request.start_time,
            end_time: request.end_time,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(EditRegionResult::from(r)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not edit region"),
    }
}

// ── delete ────────────────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRegionRequest {
    pub region_id: RegionId,
    pub region_set_id: RegionSetId,
}

#[utoipa::path(
    delete,
    path = "/regions/remove",
    tag = "regions",
    params(DeleteRegionRequest),
    responses((status = 200, description = "Region removed"))
)]
#[delete("/remove")]
pub async fn remove_region(
    role: RoleContext,
    request: web::Query<DeleteRegionRequest>,
    app_state: web::Data<RegionsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state
        .regions_service
        .delete_region(DeleteRegionParams {
            region_id: request.region_id,
            region_set_id: request.region_set_id,
        })
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Region deleted"),
        Err(_e) => HttpResponse::InternalServerError().body("Could not delete region"),
    }
}

// ── copy ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct CopyRegionRequest {
    pub source_region_id: RegionId,
    pub source_region_set_id: RegionSetId,
    pub source_track_id: TrackId,
    pub destination_region_set_id: RegionSetId,
    pub destination_track_id: TrackId,
    pub copy_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyRegionResult {
    pub region_set: RegionSetResult,
}

impl From<DbRegionSet> for CopyRegionResult {
    fn from(db: DbRegionSet) -> Self {
        CopyRegionResult {
            region_set: RegionSetResult::from(db),
        }
    }
}

#[utoipa::path(
    post,
    path = "/regions/copy",
    tag = "regions",
    params(CopyRegionRequest),
    responses((status = 200, description = "Region copied", body = serde_json::Value))
)]
#[post("/copy")]
pub async fn copy_region(
    role: RoleContext,
    request: web::Query<CopyRegionRequest>,
    app_state: web::Data<RegionsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request.into_inner();
    match app_state
        .regions_service
        .copy_region(CopyRegionParams {
            copy_name: request.copy_name,
            source_region_id: request.source_region_id,
            source_region_set_id: request.source_region_set_id,
            source_track_id: request.source_track_id,
            destination_region_set_id: request.destination_region_set_id,
            destination_track_id: request.destination_track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(CopyRegionResult::from(r)),
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(format!("Could not copy region with reason {e}"))
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_region)
        .service(remove_region)
        .service(edit_region)
        .service(copy_region);
}

use std::marker::PhantomData;

use actix_web::{delete, patch, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use domain::db::{DbRegion, RegionId, RegionSetId, TrackId};
use domain::{
    region_set::region_set_subtree::RegionSetSubtree, regions::region_subtree::RegionSubtree,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    middlewares::membership::membership_context::RoleContext,
    regions::{
        regions_app_data::RegionsAppData,
        regions_provider::{
            AddRegionParams, CopyRegionParams, DeleteRegionParams, EditRegionParams, EndTimePolicy,
        },
    },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionResult {
    pub region_id: RegionId,
    pub region_set_id: RegionSetId,
    pub name: String,
    pub start_time: f32,
    pub end_time: f32,
    pub created_at: DateTime<Utc>,
}

impl From<DbRegion> for RegionResult {
    fn from(db: DbRegion) -> Self {
        RegionResult {
            region_id: db.region_id,
            region_set_id: db.region_set_id,
            name: db.name,
            start_time: db.start_time_seconds,
            end_time: db.end_time_seconds,
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
    pub end_time: Option<f32>,
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionSetResult {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub track_length: f32,
    pub name: String,
    pub regions: Vec<RegionSubtree>,
}

impl From<RegionSetSubtree> for RegionSetResult {
    fn from(region_set: RegionSetSubtree) -> Self {
        RegionSetResult {
            region_set_id: region_set.region_set_id,
            track_id: region_set.track_id,
            track_length: region_set.track_length,
            name: region_set.name,
            regions: region_set.regions,
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
            end_time_policy: request
                .end_time
                .map(EndTimePolicy::Explicit)
                .unwrap_or(EndTimePolicy::NextRegionOrEnd),
        })
        .await
    {
        Ok(r) => HttpResponse::Created().json(RegionSetResult::from(r)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not add region"),
    }
}

// ── edit ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EditRegionRequest {
    pub region_id: RegionId,
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
    pub name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRegionResult {
    pub region: RegionResult,
}

impl From<DbRegion> for EditRegionResult {
    fn from(db: DbRegion) -> Self {
        EditRegionResult {
            region: RegionResult::from(db),
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
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(RegionSetResult::from(r)),
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
    pub region: RegionResult,
}

impl From<DbRegion> for CopyRegionResult {
    fn from(db: DbRegion) -> Self {
        CopyRegionResult {
            region: RegionResult::from(db),
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
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Could not copy region with reason {e}")),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_region)
        .service(remove_region)
        .service(edit_region)
        .service(copy_region);
}

use actix_web::{delete, get, patch, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use domain::{
    actors::messages::region_set::{
        copy_region_set::CopyRegionSet,
        create_region_set::CreateRegionSet,
        delete_region_set::DeleteRegionSet,
        edit_region_set::EditRegionSet,
        get_region_set::GetRegionSet,
        get_region_sets_for_track::{
            GetRegionSetsForTrack,
            GetRegionSetsForTrackResult as ActorGetRegionSetsForTrackResult,
        },
        get_regions_sets::{GetRegionSets, GetRegionSetsResult as ActorGetRegionSetsResult},
    },
    db::{DbRegionSet, RegionSetId, TrackId},
    region_set::region_set_subtree::RegionSetSubtree,
    regions::region_subtree::RegionSubtree,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

use crate::{
    app_data::AppData, controllers::utils::get_user_actor_internal,
    dtos::authenticated_user::AuthenticatedUser,
};

// ── create ────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateRegionSetParams {
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
    request_body = CreateRegionSetParams,
    responses(
        (status = 201, description = "Region Set Created", body = serde_json::Value),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/create")]
pub async fn create_region_set(
    user: AuthenticatedUser,
    request: web::Json<CreateRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_e) => return HttpResponse::NotFound().body("User not found"),
    };

    match resolved_user
        .ask(CreateRegionSet {
            name: request.name,
            track_id: request.track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Created().json(CreateRegionSetResult::from(r.region_set)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not create region set"),
    }
}

// ── edit ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EditRegionSetParams {
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
    request_body = EditRegionSetParams,
    responses((status = 200, description = "Region set edited", body = serde_json::Value))
)]
#[patch("/edit")]
pub async fn edit_region_set(
    user: AuthenticatedUser,
    request: web::Json<EditRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_e) => return HttpResponse::NotFound().body("User not found"),
    };

    match resolved_user
        .ask(EditRegionSet {
            name: request.name,
            region_set_id: request.region_set_id,
            track_id: request.track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(EditRegionSetResult::from(r.region_set)),
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
    user: AuthenticatedUser,
    request: web::Query<GetRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_e) => return HttpResponse::NotFound().body("User not found"),
    };

    match resolved_user
        .ask(GetRegionSet {
            region_set_id: request.region_set_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(GetRegionSetResult::from(r.region_set)),
        Err(_e) => HttpResponse::InternalServerError().body("Could not find region set"),
    }
}

// ── get-all ───────────────────────────────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRegionSetsResult {
    pub track_region_sets_map: HashMap<TrackId, Vec<GetRegionSetResult>>,
}

impl From<ActorGetRegionSetsResult> for GetRegionSetsResult {
    fn from(r: ActorGetRegionSetsResult) -> Self {
        GetRegionSetsResult {
            track_region_sets_map: r
                .track_region_sets_map
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(GetRegionSetResult::from).collect()))
                .collect(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/region-sets/get-all",
    tag = "region-sets",
    responses((status = 200, description = "All region sets", body = serde_json::Value))
)]
#[get("/get-all")]
pub async fn get_region_sets(
    user: AuthenticatedUser,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_e) => return HttpResponse::NotFound().body("User not found"),
    };

    match resolved_user.ask(GetRegionSets {}).await {
        Ok(r) => HttpResponse::Ok().json(GetRegionSetsResult::from(r)),
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

impl From<ActorGetRegionSetsForTrackResult> for GetRegionSetsForTrackResult {
    fn from(r: ActorGetRegionSetsForTrackResult) -> Self {
        GetRegionSetsForTrackResult {
            track_id: r.track_id,
            region_sets: r.region_sets.into_iter().map(GetRegionSetResult::from).collect(),
        }
    }
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
    user: AuthenticatedUser,
    request: web::Query<GetRegionsForTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_e) => return HttpResponse::NotFound().body("User not found"),
    };

    match resolved_user
        .ask(GetRegionSetsForTrack {
            track_id: request.track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(GetRegionSetsForTrackResult::from(r)),
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
    user: AuthenticatedUser,
    request: web::Query<DeleteRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_e) => return HttpResponse::NotFound().body("User not found"),
    };

    match resolved_user
        .ask(DeleteRegionSet {
            region_set_id: request.region_set_id,
        })
        .await
    {
        Ok(_r) => HttpResponse::Ok().body("Region set deleted"),
        Err(_e) => HttpResponse::InternalServerError().body("Could not delete region set"),
    }
}

// ── copy ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CopyRegionSetParams {
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
    request_body = CopyRegionSetParams,
    responses((status = 200, description = "Region set copied", body = serde_json::Value))
)]
#[post("/copy-region-set")]
pub async fn copy_region_set(
    user: AuthenticatedUser,
    request_raw: web::Json<CopyRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request_raw.into_inner();

    let user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_e) => return HttpResponse::NotFound().body("User not found"),
    };

    match user
        .ask(CopyRegionSet {
            region_set_id: request.region_set_id,
            region_set_copy_name: request.copy_region_set_name,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(CopyRegionSetResult::from(r.region_set_subtree)),
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

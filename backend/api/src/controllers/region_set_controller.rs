use actix_web::{delete, get, patch, post, web, HttpResponse};
use domain::{
    actors::messages::region_set::{
        copy_region_set::CopyRegionSet, create_region_set::CreateRegionSet,
        delete_region_set::DeleteRegionSet, edit_region_set::EditRegionSet,
        get_region_set::GetRegionSet, get_region_sets_for_track::GetRegionSetsForTrack,
        get_regions_sets::GetRegionSets,
    },
    db::{RegionSetId, TrackId},
    regions::region_set::RegionSet,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    app_data::AppData, controllers::utils::get_user_actor_internal,
    dtos::authenticated_user::AuthenticatedUser,
};
#[derive(Deserialize, ToSchema)]
pub struct CreateRegionSetParams {
    #[serde(rename = "trackId")]
    pub track_id: TrackId,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct AddRegionResult {
    #[serde(rename = "regionSet")]
    region_set: RegionSet,
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

    let rez = match resolved_user
        .ask(CreateRegionSet {
            name: request.name,
            track_id: request.track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Created().json(AddRegionResult {
            region_set: r.region_set,
        }),
        Err(_e) => return HttpResponse::InternalServerError().body("Could not create region set"),
    };
    rez
}

#[derive(Deserialize, ToSchema)]
pub struct EditRegionSetParams {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct EditRegionSetResult {
    #[serde(rename = "regionSet")]
    pub region_set: RegionSet,
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

    let rez = match resolved_user
        .ask(EditRegionSet {
            name: request.name,
            region_set_id: request.region_set_id,
            track_id: request.track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(EditRegionSetResult {
            region_set: r.region_set,
        }),
        Err(_e) => return HttpResponse::InternalServerError().body("Could not edit region set"),
    };
    rez
}

#[derive(Deserialize, IntoParams)]
pub struct GetRegionSetParams {
    pub region_set_id: RegionSetId,
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

    let rez = match resolved_user
        .ask(GetRegionSet {
            region_set_id: request.region_set_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(_e) => return HttpResponse::InternalServerError().body("Could not find  region set"),
    };
    rez
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

    let rez: HttpResponse = match resolved_user.ask(GetRegionSets {}).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(_e) => return HttpResponse::InternalServerError().body("Could not edit region"),
    };
    rez
}

#[derive(Deserialize, IntoParams)]
pub struct GetRegionsForTrackParams {
    #[serde(rename = "trackId")]
    pub track_id: TrackId,
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

    let rez = match resolved_user
        .ask(GetRegionSetsForTrack {
            track_id: request.track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(_e) => return HttpResponse::InternalServerError().body("Could not edit region"),
    };
    rez
}

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

    let rez = match resolved_user
        .ask(DeleteRegionSet {
            region_set_id: request.region_set_id,
        })
        .await
    {
        Ok(_r) => HttpResponse::Ok().body("Region deleted"),
        Err(_e) => return HttpResponse::InternalServerError().body("Could not delete region set"),
    };
    rez
}

#[derive(Deserialize, ToSchema)]
pub struct CopyRegionSetParams {
    #[serde(rename = "regionSetId")]
    pub region_set_id: RegionSetId,

    #[serde(rename = "copyName")]
    pub copy_region_set_name: String,
}

#[derive(Serialize)]
pub struct CopyRegionSetResult {
    #[serde(rename = "regionSet")]
    pub region_set: RegionSet,
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

    let rez = match user
        .ask(CopyRegionSet {
            region_set_id: request.region_set_id,
            region_set_copy_name: request.copy_region_set_name,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json(CopyRegionSetResult {
            region_set: smth.region_set,
        }),
        Err(_e) => return HttpResponse::InternalServerError().body("Could not copy track"),
    };
    rez
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

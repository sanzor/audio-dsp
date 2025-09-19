use actix_web::{delete, get, patch, post, web, HttpResponse};
use domain::{
    actors::messages::region_set::{
        create_region_set::CreateRegionSet, delete_region_set::DeleteRegionSet,
        edit_region_set::EditRegionSet, get_region_set::GetRegionSet, get_region_sets_for_track::GetRegionSetsForTrack, get_regions_sets::GetRegionSets,
    },
    regions::region_set::RegionSet,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_data::AppData, controllers::utils::get_user_actor_internal,
    dtos::authenticated_user::AuthenticatedUser,
};
#[derive(Deserialize)]
pub struct CreateRegionSetParams {
    pub track_id: String,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct AddRegionResult {
    region_set: RegionSet,
}
#[post("/create")]
pub async fn create_region_set(
    user: AuthenticatedUser,
    request: web::Json<CreateRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
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
        Err(e) => return HttpResponse::InternalServerError().body("Could not create region set"),
    };
    rez
}

#[derive(Deserialize)]
pub struct EditRegionSetParams {
    pub region_set_id: String,
    pub track_id: String,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct EditRegionSetResult {
    pub region_set: RegionSet,
}

#[patch("/edit")]
pub async fn edit_region_set(
    user: AuthenticatedUser,
    request: web::Json<EditRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
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
        Err(e) => return HttpResponse::InternalServerError().body("Could not edit region set"),
    };
    rez
}



#[derive(Deserialize)]
pub struct GetRegionSetParams {
    pub region_set_id: String,
}

#[get("/get")]
pub async fn get_region_set(
    user: AuthenticatedUser,
    request: web::Query<GetRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match resolved_user
        .ask(GetRegionSet {
            region_set_id: request.region_set_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => return HttpResponse::InternalServerError().body("Could not find  region set"),
    };
    rez
}

#[get("/get-all")]
pub async fn get_region_sets(
    user: AuthenticatedUser,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez: HttpResponse = match resolved_user
        .ask(GetRegionSets {
            
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => return HttpResponse::InternalServerError().body("Could not edit region"),
    };
    rez
}



#[derive(Deserialize)]
pub struct GetRegionsForTrackParams {
    pub track_id: String,
}

#[get("/get-all-for-track")]
pub async fn get_region_sets_for_track(
    user: AuthenticatedUser,
    request: web::Query<GetRegionsForTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match resolved_user
        .ask(GetRegionSetsForTrack {
            track_id: request.track_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => return HttpResponse::InternalServerError().body("Could not edit region"),
    };
    rez
}


#[derive(Deserialize)]
pub struct DeleteRegionSetParams {
    pub region_set_id: String,
}

#[delete("/delete")]
pub async fn delete_region_set(
    user: AuthenticatedUser,
    request: web::Query<DeleteRegionSetParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match resolved_user
        .ask(DeleteRegionSet {
            region_set_id: request.region_set_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().body("Region deleted"),
        Err(e) => return HttpResponse::InternalServerError().body("Could not delete region set"),
    };
    rez
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_region_set)
        .service(delete_region_set)
        .service(edit_region_set)
        .service(get_region_set);
}

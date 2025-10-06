use actix_web::{delete, get, patch, post, web, HttpResponse};
use domain::{
    actors::messages::regions::{
        add_region::{AddRegion, EndTimePolicy},
        copy_region::CopyRegion,
        delete_region::DeleteRegion,
        edit_region::EditRegion,
    },
    regions::region_set::RegionSet,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_data::AppData, controllers::utils::get_user_actor_internal,
    dtos::authenticated_user::AuthenticatedUser,
};
#[derive(Deserialize)]
pub struct AddRegionParams {
    pub track_id: String,
    pub start_time: f32,
    pub name: String,
    pub end_time_policy: String,
}
#[derive(Serialize)]
pub struct AddRegionResult {
    region_set: RegionSet,
}
#[post("/add")]
pub async fn add_region(
    user: AuthenticatedUser,
    request: web::Json<AddRegionParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match resolved_user
        .ask(AddRegion {
            name: request.name,
            region_set_id: request.track_id,
            start_time: request.start_time,
            end_time_policy: EndTimePolicy::NextRegionOrEnd,
        })
        .await
    {
        Ok(r) => HttpResponse::Created().json(AddRegionResult {
            region_set: r.region_set,
        }),
        Err(e) => return HttpResponse::InternalServerError().body("Could not add region"),
    };
    rez
}

#[derive(Deserialize)]
pub struct EditRegionParams {
    pub region_id: String,
    pub region_set_id: String,
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct EditRegionResult {
    pub region_set: RegionSet,
}

#[patch("/edit")]
pub async fn edit_region(
    user: AuthenticatedUser,
    request: web::Json<EditRegionParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match resolved_user
        .ask(EditRegion {
            name: request.name,
            region_set_id: request.region_set_id,
            region_id: request.region_id,
            start_time: request.start_time,
            end_time: request.end_time,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(EditRegionResult {
            region_set: r.region_set,
        }),
        Err(e) => return HttpResponse::InternalServerError().body("Could not edit region"),
    };
    rez
}

#[derive(Deserialize)]
pub struct DeleteRegionParams {
    pub region_id: String,
    pub region_set_id: String,
}

#[delete("/remove")]
pub async fn remove_region(
    user: AuthenticatedUser,
    request: web::Query<DeleteRegionParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match resolved_user
        .ask(DeleteRegion {
            region_id: request.region_id,
            region_set_id: request.region_set_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().body("Region deleted"),
        Err(e) => return HttpResponse::InternalServerError().body("Could not edit region"),
    };
    rez
}

#[derive(Deserialize)]
pub struct CopyRegionParams {
    pub region_id: String,
    pub region_set_id: String,
    pub copy_name: String,
}

#[post("/copy")]
pub async fn copy_region(
    user: AuthenticatedUser,
    request: web::Query<CopyRegionParams>,
    app_state: web::Data<AppData>,
) -> HttpResponselet {
    let request = request.into_inner();
    let resolved_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };
    let rez = match resolved_user.ask(CopyRegion {
        copy_name: request.copy_name,
        region_id: request.region_id,
        region_set_id: request.region_set_id,
    }) {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Could not copy region with reason {:?}", e))
        }
    }?;
    rez
}
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_region)
        .service(remove_region)
        .service(edit_region)
        .service(copy_region);
}

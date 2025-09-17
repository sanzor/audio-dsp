use actix_web::{delete, patch, post,get, web, HttpResponse};
use domain::actors::messages::regions::{add_region::AddRegion, delete_region::DeleteRegion, edit_region::EditRegion, get_regions::GetRegions};
use serde::{Deserialize, Serialize};

use crate::{
    app_data::AppData, controllers::utils::get_user_actor_internal,
    dtos::authenticated_user::AuthenticatedUser,
};
#[derive(Deserialize)]
pub struct AddRegionParams{
    pub track_id:String,
    pub start_time:f32,
    pub end_time:Option<f32>,
    pub name:String
}
#[derive(Serialize)]
pub struct AddRegionResult{
    region_id:String,
    track_id:String,
    name:String,
    start_time:f32,
    end_time:f32
}
#[post("/add")]
pub async fn add_region(user:AuthenticatedUser,request:web::Json<AddRegionParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=request.into_inner();
    let resolved_user= match get_user_actor_internal(&user.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found"),
    };
    
    let rez=match resolved_user.ask(AddRegion{
        name:request.name,
        track_id:request.track_id,
        start_time:request.start_time,
        end_time:request.end_time
    }).await{
        Ok(r)=>HttpResponse::Created().json(AddRegionResult{
            track_id:r.track_id,
            region_id:r.region_id,
            name:r.name,
            start_time:r.start_time,
            end_time:r.end_time
        }),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not add region")
    };
    rez
}

#[derive(Deserialize)]
pub struct EditRegionParams{
    pub region_id:String,
    pub start_time:Option<f32>,
    pub end_time:Option<f32>,
    pub name:Option<String>
}

#[derive(Serialize)]
pub struct EditRegionResult{
    pub region_id:String,
    pub track_id:String,
    pub start_time:f32,
    pub end_time:f32,
    pub name:String
}

#[patch("/edit")]
pub async fn edit_region(user:AuthenticatedUser,request:web::Json<EditRegionParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=request.into_inner();
    let resolved_user= match get_user_actor_internal(&user.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found"),
    };
    
    let rez=match resolved_user.ask(EditRegion{
        name:request.name,
        region_id:request.region_id,
        start_time:request.start_time,
        end_time:request.end_time
    }).await{
        Ok(r)=>HttpResponse::Ok().json(EditRegionResult{
            name:r.name,
            end_time:r.end_time,
            start_time:r.start_time,
            region_id:r.region_id,
            track_id:r.track_id
        }),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not edit region")
    };
    rez
}

#[derive(Deserialize)]
pub struct DeleteRegionParams{
    pub region_id:String
}

#[delete("/delete")]
pub async fn delete_region(user:AuthenticatedUser,request:web::Query<DeleteRegionParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=request.into_inner();
    let resolved_user= match get_user_actor_internal(&user.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found"),
    };
    
    let rez=match resolved_user.ask(DeleteRegion{
        region_id:request.region_id
    }).await{
        Ok(r)=>HttpResponse::Ok().body("Region deleted"),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not edit region")
    };
    rez
}

#[derive(Deserialize)]
pub struct GetRegionsParams{
    pub track_id:String
}

#[get("/get-regions")]
pub async fn get_regions(user:AuthenticatedUser,request:web::Query<GetRegionsParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=request.into_inner();
    let resolved_user= match get_user_actor_internal(&user.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found"),
    };
    
    let rez=match resolved_user.ask(GetRegions{
        track_id:request.track_id
    }).await{
        Ok(r)=>HttpResponse::Ok().json(r) ,
        Err(e)=>return HttpResponse::InternalServerError().body("Could not edit region")
    };
    rez
}

pub fn init(cfg:&mut web::ServiceConfig){
    cfg.service(add_region)
        .service(delete_region)
        .service(edit_region)
        .service(get_regions);
}
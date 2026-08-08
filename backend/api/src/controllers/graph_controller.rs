use actix_web::{HttpResponse, cookie::time::format_description::Component::Period, delete, get, patch, post, put, web};
use chrono::{DateTime, Utc};
use domain::{
    db::{db_graph::DbGraph, GraphId, RegionId},
    graphs::{
        add_graph_params::AddGraphParams,
        copy_graph_params::CopyGraphParams,
        delete_graph_params::DeleteGraphParams,
        edit_graph_params::EditGraphParams,
        save_graph_state_params::SaveGraphStateParams,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

use crate::{
    domain::service_error::ServiceError,
    graphs::graphs_app_data::GraphsAppData,
    middlewares::membership::membership_context::RoleContext,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphResult {
    id: GraphId,
    region_id: RegionId,
    name: String,
    version: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    repr: Value,
}

#[derive(Serialize)]
pub struct GraphMutationResult {
    graph: GraphResult,
}

impl GraphResult {
    fn from_db_graph(graph: DbGraph) -> Result<Self, ServiceError> {
        let region_id = graph
            .region_id
            .ok_or_else(|| ServiceError::Internal("graph missing region_id".to_string()))?;

        Ok(Self {
            id: graph.graph_id,
            region_id,
            name: graph.name,
            version: graph.version,
            created_at: graph.created_at,
            updated_at: graph.updated_at,
            repr: graph.graph_state,
        })
    }
}

fn map_service_error(error: ServiceError) -> HttpResponse {
    match error {
        ServiceError::Internal(err) => HttpResponse::InternalServerError().body(err),
        ServiceError::Validation(err) => HttpResponse::BadRequest().body(err),
        ServiceError::NotFound => HttpResponse::NotFound().finish(),
        ServiceError::Conflict(err) => HttpResponse::Conflict().body(err),
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateGraphRequest {
    pub region_id: RegionId,
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/graphs/create",
    tag = "graphs",
    request_body = CreateGraphRequest,
    responses((status = 201, description = "Graph created", body = serde_json::Value))
)]
#[post("/create")]
pub async fn create_graph(
    role: RoleContext,
    request: web::Json<CreateGraphRequest>,
    app_state: web::Data<GraphsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let request = request.into_inner();
    match app_state
        .graphs_service
        .create_graph(AddGraphParams {
            region_id: request.region_id,
            name: request.name,
        })
        .await
    {
        Ok(graph) => match GraphResult::from_db_graph(graph) {
            Ok(graph) => HttpResponse::Created().json(GraphMutationResult { graph }),
            Err(err) => map_service_error(err),
        },
        Err(err) => map_service_error(err),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct EditGraphRequest {
    #[serde(alias = "graph_id")]
    pub id: GraphId,
    pub name: String,
}

#[utoipa::path(
    patch,
    path = "/graphs/edit",
    tag = "graphs",
    request_body = EditGraphRequest,
    responses((status = 200, description = "Graph edited", body = serde_json::Value))
)]
#[patch("/edit")]
pub async fn edit_graph(
    role: RoleContext,
    request: web::Json<EditGraphRequest>,
    app_state: web::Data<GraphsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let request = request.into_inner();
    match app_state
        .graphs_service
        .edit_graph(EditGraphParams {
            graph_id: request.id,
            name: Some(request.name),
        })
        .await
    {
        Ok(graph) => match GraphResult::from_db_graph(graph) {
            Ok(graph) => HttpResponse::Ok().json(GraphMutationResult { graph }),
            Err(err) => map_service_error(err),
        },
        Err(err) => map_service_error(err),
    }
}

#[derive(Deserialize, IntoParams)]
pub struct GetGraphRequest {
    pub graph_id: GraphId,
}

#[utoipa::path(
    get,
    path = "/graphs/get-graph",
    tag = "graphs",
    params(GetGraphRequest),
    responses((status = 200, description = "Graph fetched", body = serde_json::Value))
)]
#[get("/get-graph")]
pub async fn get_graph(
    role: RoleContext,
    request: web::Query<GetGraphRequest>,
    app_state: web::Data<GraphsAppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    match app_state.graphs_service.get_graph(&request.graph_id).await {
        Ok(graph) => match GraphResult::from_db_graph(graph) {
            Ok(graph) => HttpResponse::Ok().json(graph),
            Err(err) => map_service_error(err),
        },
        Err(err) => map_service_error(err),
    }
}

#[derive(Deserialize, IntoParams)]
pub struct DeleteGraphRequest {
    pub graph_id: GraphId,
}

#[utoipa::path(
    delete,
    path = "/graphs/remove",
    tag = "graphs",
    params(DeleteGraphRequest),
    responses((status = 204, description = "Graph removed"))
)]
#[delete("/remove")]
pub async fn remove_graph(
    role: RoleContext,
    request: web::Query<DeleteGraphRequest>,
    app_state: web::Data<GraphsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    match app_state
        .graphs_service
        .delete_graph(DeleteGraphParams {
            graph_id: request.graph_id,
        })
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => map_service_error(err),
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CopyGraphRequest {
    pub destination_region_id: RegionId,
    pub source_graph_id: GraphId,
    pub copy_name: String,
}

#[utoipa::path(
    post,
    path = "/graphs/copy",
    tag = "graphs",
    request_body = CopyGraphRequest,
    responses((status = 201, description = "Graph copied", body = serde_json::Value))
)]
#[post("/copy")]
pub async fn copy_graph(
    role: RoleContext,
    request: web::Json<CopyGraphRequest>,
    app_state: web::Data<GraphsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let request = request.into_inner();
    match app_state
        .graphs_service
        .copy_graph(CopyGraphParams {
            source_graph_id: request.source_graph_id,
            destination_region_id: request.destination_region_id,
            copy_name: request.copy_name,
        })
        .await
    {
        Ok(graph) => match GraphResult::from_db_graph(graph) {
            Ok(graph) => HttpResponse::Created().json(GraphMutationResult { graph }),
            Err(err) => map_service_error(err),
        },
        Err(err) => map_service_error(err),
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SaveGraphStateRequest {
    pub graph_id: GraphId,
    pub state: String,
}

#[utoipa::path(
    put,
    path = "/graphs/save-state",
    tag = "graphs",
    request_body = SaveGraphStateRequest,
    responses((status = 200, description = "Graph state saved", body = serde_json::Value))
)]
#[put("/save-state")]
pub async fn save_graph_state(
    role: RoleContext,
    request: web::Json<SaveGraphStateRequest>,
    app_state: web::Data<GraphsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let request = request.into_inner();
    let state: Value = match serde_json::from_str(&request.state) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().body("invalid JSON in state"),
    };

    match app_state
        .graphs_service
        .save_graph_state(SaveGraphStateParams {
            graph_id: request.graph_id,
            state,
        })
        .await
    {
        Ok(graph) => match GraphResult::from_db_graph(graph) {
            Ok(graph) => HttpResponse::Ok().json(GraphMutationResult { graph }),
            Err(err) => map_service_error(err),
        },
        Err(err) => map_service_error(err),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_graph)
        .service(edit_graph)
        .service(save_graph_state)
        .service(get_graph)
        .service(remove_graph)
        .service(copy_graph);
}

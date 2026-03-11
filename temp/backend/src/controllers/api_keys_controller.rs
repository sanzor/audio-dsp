use crate::api_keys::api_keys_app_data::ApiKeysAppData;
use crate::api_keys::create_api_key_params::CreateApiKeyParams;
use crate::api_keys::update_api_key_params::UpdateApiKeyParams;
use crate::domain::db::db_api_key::{ApiKeyId, DbApiKey};
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::middlewares::organization::organization_context::OrganizationContext;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ApiKeyResult {
    pub id: ApiKeyId,
    pub org_id: OrganizationId,
    pub key: String,
    pub label: Option<String>,
    pub is_active: bool,
    pub created_by: Option<i64>,
}

impl From<DbApiKey> for ApiKeyResult {
    fn from(value: DbApiKey) -> Self {
        Self {
            id: value.id,
            org_id: value.org_id,
            key: value.key,
            label: value.label,
            is_active: value.is_active,
            created_by: value.created_by,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ApiKeysResult {
    pub api_keys: Vec<ApiKeyResult>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ApiKeyPath {
    pub api_key_id: ApiKeyId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateApiKeyInput {
    pub label: Option<String>,
    pub created_by: Option<UserId>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateApiKeyResult {
    pub api_key: ApiKeyResult,
}

#[utoipa::path(
    post,
    path = "/api-keys",
    tag = "ApiKeys",
    request_body = CreateApiKeyInput,
    responses(
        (status = 201, description = "API key created", body = CreateApiKeyResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("")]
pub async fn create_api_key(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    payload: web::Json<CreateApiKeyInput>,
    app_state: web::Data<ApiKeysAppData>,
) -> HttpResponse {
    if !perms.has("api_keys:create") {
        return HttpResponse::Forbidden().body("missing required permission: api_keys:create");
    }
    info!(org_id = org_ctx.id, "create api key request received");
    let input = payload.into_inner();

    let params = CreateApiKeyParams {
        org_id: org_ctx.id,
        label: input.label,
        created_by: input.created_by,
    };

    match app_state.api_keys_provider.create_api_key(params).await {
        Ok(result) => HttpResponse::Created().json(CreateApiKeyResult {
            api_key: ApiKeyResult::from(result.api_key),
        }),
        Err(e) => {
            error!(error = %e, "create api key failed");
            HttpResponse::InternalServerError().body("failed to create api key")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateApiKeyInput {
    pub id: ApiKeyId,
    pub label: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateApiKeyResult {
    pub api_key: ApiKeyResult,
}

#[utoipa::path(
    put,
    path = "/api-keys/{api_key_id}",
    tag = "ApiKeys",
    request_body = UpdateApiKeyInput,
    params(
        ("api_key_id" = u64, Path, description = "API key id"),
    ),
    responses(
        (status = 200, description = "API key updated", body = UpdateApiKeyResult),
        (status = 404, description = "API key not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/{api_key_id}")]
pub async fn update_api_key(
    perms: PermissionsContext,
    path: web::Path<ApiKeyPath>,
    payload: web::Json<UpdateApiKeyInput>,
    app_state: web::Data<ApiKeysAppData>,
) -> HttpResponse {
    if !perms.has("api_keys:update") {
        return HttpResponse::Forbidden().body("missing required permission: api_keys:update");
    }
    let api_key_id = path.into_inner().api_key_id;
    info!(api_key_id, "update api key request received");
    let input = payload.into_inner();

    if input.label.is_none() && input.is_active.is_none() {
        warn!(api_key_id, "update api key rejected: no fields provided");
        return HttpResponse::BadRequest().body("at least one field is required");
    }

    let params = UpdateApiKeyParams {
        id: api_key_id,

        label: input.label,
        is_active: input.is_active,
    };

    match app_state
        .api_keys_provider
        .update_api_key(api_key_id, params)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(UpdateApiKeyResult {
            api_key: ApiKeyResult::from(result.api_key),
        }),
        Ok(None) => HttpResponse::NotFound().body("api key not found"),
        Err(e) => {
            error!(api_key_id, error = %e, "update api key failed");
            HttpResponse::InternalServerError().body("failed to update api key")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api-keys/{api_key_id}",
    tag = "ApiKeys",
    params(
        ("api_key_id" = u64, Path, description = "API key id"),
    ),
    responses(
        (status = 204, description = "API key deleted"),
        (status = 404, description = "API key not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{api_key_id}")]
pub async fn delete_api_key(
    perms: PermissionsContext,
    path: web::Path<ApiKeyPath>,
    app_state: web::Data<ApiKeysAppData>,
) -> HttpResponse {
    if !perms.has("api_keys:delete") {
        return HttpResponse::Forbidden().body("missing required permission: api_keys:delete");
    }
    let api_key_id = path.into_inner().api_key_id;
    info!(api_key_id, "delete api key request received");

    match app_state.api_keys_provider.delete_api_key(api_key_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("api key not found"),
        Err(e) => {
            error!(api_key_id, error = %e, "delete api key failed");
            HttpResponse::InternalServerError().body("failed to delete api key")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetApiKeyResult {
    pub api_key: ApiKeyResult,
}

#[utoipa::path(
    get,
    path = "/api-keys/{api_key_id}",
    tag = "ApiKeys",
    params(
        ("api_key_id" = u64, Path, description = "API key id"),
    ),
    responses(
        (status = 200, description = "API key retrieved", body = GetApiKeyResult),
        (status = 404, description = "API key not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{api_key_id}")]
pub async fn get_api_key(
    perms: PermissionsContext,
    path: web::Path<ApiKeyPath>,
    app_state: web::Data<ApiKeysAppData>,
) -> HttpResponse {
    if !perms.has("api_keys:read") {
        return HttpResponse::Forbidden().body("missing required permission: api_keys:read");
    }
    let api_key_id = path.into_inner().api_key_id;
    info!(api_key_id, "get api key request received");

    match app_state.api_keys_provider.get_api_key(api_key_id).await {
        Ok(Some(result)) => HttpResponse::Ok().json(GetApiKeyResult {
            api_key: ApiKeyResult::from(result.api_key),
        }),
        Ok(None) => HttpResponse::NotFound().body("api key not found"),
        Err(e) => {
            error!(api_key_id, error = %e, "get api key failed");
            HttpResponse::InternalServerError().body("failed to get api key")
        }
    }
}

#[utoipa::path(
    get,
    path = "/api-keys",
    tag = "ApiKeys",
    responses(
        (status = 200, description = "API keys retrieved", body = ApiKeysResult),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn get_all_api_keys(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    app_state: web::Data<ApiKeysAppData>,
) -> HttpResponse {
    if !perms.has("api_keys:read") {
        return HttpResponse::Forbidden().body("missing required permission: api_keys:read");
    }
    info!(org_id = org_ctx.id, "list api keys request received");

    match app_state
        .api_keys_provider
        .list_api_keys_by_org(org_ctx.id)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(ApiKeysResult {
            api_keys: result.into_iter().map(ApiKeyResult::from).collect(),
        }),
        Err(e) => {
            error!(error = %e, "list api keys failed");
            HttpResponse::InternalServerError().body("failed to fetch api keys")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_api_key)
        .service(update_api_key)
        .service(delete_api_key)
        .service(get_api_key)
        .service(get_all_api_keys);
}

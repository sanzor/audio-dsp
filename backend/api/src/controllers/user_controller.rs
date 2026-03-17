use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::users::user_result::UserResult;
use crate::users::{
    create_user_params::CreateUserParams, update_user_params::UpdateUserParams,
    users_app_data::UsersAppData,
};

fn map_service_error(err: ServiceError) -> HttpResponse {
    match err {
        ServiceError::NotFound => HttpResponse::NotFound().body("not found"),
        ServiceError::Conflict(msg) => HttpResponse::Conflict().body(msg),
        ServiceError::Validation(msg) => HttpResponse::BadRequest().body(msg),
        ServiceError::Internal(msg) => {
            error!(error = %msg, "internal error");
            HttpResponse::InternalServerError().body("internal server error")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UsersResult {
    pub users: Vec<UserResult>,
    pub total: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UserListQuery {
    pub index: Option<i64>,
    pub count: Option<i64>,
    pub search: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateUserInput {
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateUserResult {
    pub user: UserResult,
}

#[utoipa::path(
    post,
    path = "/users",
    tag = "Users",
    request_body = CreateUserInput,
    responses(
        (status = 201, description = "User created", body = CreateUserResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("")]
pub async fn create_user(
    perms: PermissionsContext,
    payload: web::Json<CreateUserInput>,
    app_state: web::Data<UsersAppData>,
) -> HttpResponse {
    if !perms.has("users:create") {
        return HttpResponse::Forbidden().body("missing required permission: users:create");
    }
    info!("create user request received");
    let input = payload.into_inner();
    if input.email.trim().is_empty() {
        warn!("create user rejected: missing email");
        return HttpResponse::BadRequest().body("email is required");
    }
    if input.password_hash.trim().is_empty() {
        warn!("create user rejected: missing password_hash");
        return HttpResponse::BadRequest().body("password_hash is required");
    }
    if input.full_name.trim().is_empty() {
        warn!("create user rejected: missing full_name");
        return HttpResponse::BadRequest().body("full_name is required");
    }

    let params = CreateUserParams {
        email: input.email,
        password_hash: input.password_hash,
        full_name: input.full_name,
        is_active: input.is_active,
        is_verified: Some(false),
    };

    match app_state.user_provider.create_user(params).await {
        Ok(result) => HttpResponse::Created().json(CreateUserResult { user: result.user }),
        Err(e) => map_service_error(e),
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateUserInput {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateUserResult {
    pub user: UserResult,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateUserPath {
    pub user_id: UserId,
}

#[utoipa::path(
    put,
    path = "/users/{user_id}",
    tag = "Users",
    request_body = UpdateUserInput,
    params(("user_id" = UserId, Path, description = "User id")),
    responses(
        (status = 200, description = "User updated", body = UpdateUserResult),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/{user_id}")]
pub async fn update_user(
    perms: PermissionsContext,
    path: web::Path<UpdateUserPath>,
    payload: web::Json<UpdateUserInput>,
    app_state: web::Data<UsersAppData>,
) -> HttpResponse {
    if !perms.has("users:update") {
        return HttpResponse::Forbidden().body("missing required permission: users:update");
    }
    let user_id = path.into_inner().user_id;
    info!(user_id, "update user request received");
    let input = payload.into_inner();

    if input.email.is_none()
        && input.password_hash.is_none()
        && input.full_name.is_none()
        && input.is_active.is_none()
    {
        warn!(user_id, "update user rejected: no fields provided");
        return HttpResponse::BadRequest().body("at least one field is required");
    }

    let params = UpdateUserParams {
        email: input.email,
        password_hash: input.password_hash,
        full_name: input.full_name,
        is_active: input.is_active,
        is_verified: None,
    };

    match app_state.user_provider.update_user(user_id, params).await {
        Ok(Some(result)) => HttpResponse::Ok().json(UpdateUserResult { user: result.user }),
        Ok(None) => HttpResponse::NotFound().body("user not found"),
        Err(e) => map_service_error(e),
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct DeleteUserPath {
    pub user_id: UserId,
}

#[utoipa::path(
    delete,
    path = "/users/{user_id}",
    tag = "Users",
    params(("user_id" = UserId, Path, description = "User id")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{user_id}")]
pub async fn delete_user(
    perms: PermissionsContext,
    path: web::Path<DeleteUserPath>,
    app_state: web::Data<UsersAppData>,
) -> HttpResponse {
    if !perms.has("users:delete") {
        return HttpResponse::Forbidden().body("missing required permission: users:delete");
    }
    let user_id = path.into_inner().user_id;
    info!(user_id, "delete user request received");

    match app_state.user_provider.delete_user(user_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("user not found"),
        Err(e) => map_service_error(e),
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetUserPath {
    pub user_id: UserId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetUserResult {
    pub user: UserResult,
}

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    tag = "Users",
    params(("user_id" = UserId, Path, description = "User id")),
    responses(
        (status = 200, description = "User retrieved", body = GetUserResult),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{user_id}")]
pub async fn get_user(
    perms: PermissionsContext,
    path: web::Path<GetUserPath>,
    app_state: web::Data<UsersAppData>,
) -> HttpResponse {
    if !perms.has("users:read") {
        return HttpResponse::Forbidden().body("missing required permission: users:read");
    }
    let user_id = path.into_inner().user_id;
    info!(user_id, "get user request received");

    match app_state.user_provider.get_user(user_id).await {
        Ok(Some(result)) => HttpResponse::Ok().json(GetUserResult { user: result }),
        Ok(None) => HttpResponse::NotFound().body("user not found"),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    responses(
        (status = 200, description = "Users retrieved", body = UsersResult),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn get_all_users(
    perms: PermissionsContext,
    query: web::Query<UserListQuery>,
    app_state: web::Data<UsersAppData>,
) -> HttpResponse {
    if !perms.has("users:read") {
        return HttpResponse::Forbidden().body("missing required permission: users:read");
    }
    let offset = query.index.unwrap_or(0).max(0) as usize;
    let limit = query.count.unwrap_or(20).clamp(1, 100) as usize;
    let search = query.search.as_deref().map(str::to_lowercase);
    info!(offset, limit, "list users request received");

    match app_state.user_provider.get_all_users().await {
        Ok(users) => {
            let filtered: Vec<_> = users
                .into_iter()
                .filter(|u| {
                    search.as_deref().map_or(true, |s| {
                        u.email.to_lowercase().contains(s)
                            || u.full_name.to_lowercase().contains(s)
                    })
                })
                .collect();
            let total = filtered.len() as i64;
            let page: Vec<_> = filtered.into_iter().skip(offset).take(limit).collect();
            HttpResponse::Ok().json(UsersResult { users: page, total })
        }
        Err(e) => map_service_error(e),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user)
        .service(update_user)
        .service(delete_user)
        .service(get_user)
        .service(get_all_users);
}

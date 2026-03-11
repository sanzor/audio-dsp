use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_token_bucket_usage::DbTokenBucketUsage;
use crate::domain::db::db_token_window_usage::DbTokenWindowUsage;
use crate::domain::db::usage_snapshot::UsageSnapshot;
use crate::middlewares::organization::organization_context::OrganizationContext;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::usage::usage_app_data::UsageAppData;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(tag = "type")]
pub enum UsageSnapshotResult {
    TokenBucket {
        org_id: OrganizationId,
        limit: Option<i64>,
        bucket_tokens: Option<i64>,
    },
    TokenWindow {
        org_id: OrganizationId,
        limit: Option<i64>,
        window_size_secs: Option<i64>,
        window_counter: Option<i64>,
        window_start: Option<i64>,
    },
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetUsageResponse {
    pub usage: UsageSnapshotResult,
}

fn map_usage(snapshot: UsageSnapshot) -> UsageSnapshotResult {
    match snapshot {
        UsageSnapshot::TokenBucket(DbTokenBucketUsage { org_id, limit, bucket_tokens }) => {
            UsageSnapshotResult::TokenBucket { org_id, limit, bucket_tokens }
        }
        UsageSnapshot::TokenWindow(DbTokenWindowUsage {
            org_id,
            limit,
            window_size_secs,
            window_counter,
            window_start,
        }) => UsageSnapshotResult::TokenWindow {
            org_id,
            limit,
            window_size_secs,
            window_counter,
            window_start,
        },
    }
}

#[utoipa::path(
    get,
    path = "/usage",
    tag = "Usage",
    responses(
        (status = 200, description = "Usage snapshot retrieved", body = GetUsageResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "No usage data yet"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn get_usage(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    app_state: web::Data<UsageAppData>,
) -> HttpResponse {
    if !perms.has("rate_limits:read") {
        return HttpResponse::Forbidden().body("missing required permission: rate_limits:read");
    }
    let org_id = org_ctx.id;

    match app_state.usage_provider.get_usage(org_id).await {
        Ok(Some(snapshot)) => HttpResponse::Ok().json(GetUsageResponse {
            usage: map_usage(snapshot),
        }),
        Ok(None) => HttpResponse::NotFound().body("no usage data yet"),
        Err(e) => {
            error!(org_id, error = %e, "get usage failed");
            HttpResponse::InternalServerError().body("failed to get usage")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_usage);
}

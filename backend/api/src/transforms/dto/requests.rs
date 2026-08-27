use domain::db::db_transform::TransformId;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct TransformIdPath {
    pub transform_id: TransformId,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TransformIdsRequest {
    pub ids: Vec<TransformId>,
}

#[derive(Deserialize, IntoParams)]
pub struct PaginationQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

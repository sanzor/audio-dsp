use domain::{db::TransformId, domain_user::UserId};

pub struct RequestCompileParams {
    pub user_id: UserId,
    pub transform_id: TransformId,
    pub payload: String,
}

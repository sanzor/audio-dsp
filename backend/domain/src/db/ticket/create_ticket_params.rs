use crate::{db::TransformId, domain_user::UserId};

pub struct CreateTransformDraftParams {
    pub transform_id: TransformId,
    pub user_id: UserId,
    pub source_code: String,
}

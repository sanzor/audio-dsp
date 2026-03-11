use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::users::create_user_params::CreateUserParams;
use crate::users::data_provider::user_data_provider::UserDataProvider;
use crate::users::update_user_params::UpdateUserParams;

use crate::domain::db::db_user::{DbUser, UserId};

pub struct UserDataProviderService {
    pool: PgPool,
}

impl UserDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserDataProvider for UserDataProviderService {
    async fn create_user(&self, params: CreateUserParams) -> Result<DbUser, DataError> {
        let record = sqlx::query_as::<_, DbUser>(
            "INSERT INTO users (email, password_hash, full_name, is_active, is_verified) \
             VALUES ($1, $2, $3, $4, $5) \
             RETURNING id, email, full_name, is_active, is_verified",
        )
        .bind(params.email)
        .bind(params.password_hash)
        .bind(params.full_name)
        .bind(params.is_active)
        .bind(params.is_verified)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    async fn update_user(
        &self,
        user_id: UserId,
        params: UpdateUserParams,
    ) -> Result<Option<DbUser>, DataError> {
        let user_id = i64::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".to_string()))?;
        let record = sqlx::query_as::<_, DbUser>(
            "UPDATE users \
             SET email = COALESCE($2, email), \
                 password_hash = COALESCE($3, password_hash), \
                 full_name = COALESCE($4, full_name), \
                 is_active = COALESCE($5, is_active), \
                 is_verified = COALESCE($6, is_verified), \
                 updated_at = NOW() \
             WHERE id = $1 \
             RETURNING id, email, full_name, is_active, is_verified",
        )
        .bind(user_id)
        .bind(params.email)
        .bind(params.password_hash)
        .bind(params.full_name)
        .bind(params.is_active)
        .bind(params.is_verified)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn delete_user(&self, user_id: UserId) -> Result<bool, DataError> {
        let user_id = i64::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".to_string()))?;
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_user(&self, user_id: UserId) -> Result<Option<DbUser>, DataError> {
        let user_id = i64::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".to_string()))?;
        let record = sqlx::query_as::<_, DbUser>(
            "SELECT id, email, full_name, is_active, is_verified FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn get_user_by_email(&self, email: String) -> Result<Option<DbUser>, DataError> {
        let record = sqlx::query_as::<_, DbUser>(
            "SELECT id, email, full_name, is_active, is_verified FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn get_all_users(&self) -> Result<Vec<DbUser>, DataError> {
        let records = sqlx::query_as::<_, DbUser>(
            "SELECT id, email, full_name, is_active, is_verified FROM users ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

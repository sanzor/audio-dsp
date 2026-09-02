use async_trait::async_trait;
use domain::domain_user::UserId;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::domain::db::db_user::DbUser;
use crate::users::create_user_params::CreateUserParams;
use crate::users::data_provider::user_data_provider::UserDataProvider;
use crate::users::update_user_params::UpdateUserParams;

pub struct UserDataProviderService {
    pool: PgPool,
}

impl UserDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

const USER_SELECT_COLUMNS: &str = "user_id AS id, email, name AS full_name, is_active, is_verified";

#[async_trait]
impl UserDataProvider for UserDataProviderService {
    async fn create_user(&self, params: CreateUserParams) -> Result<DbUser, DataError> {
        let query = format!(
            "INSERT INTO users (email, password_hash, name, is_active, is_verified) \
             VALUES ($1, $2, $3, $4, $5) \
             RETURNING {USER_SELECT_COLUMNS}"
        );

        let record = sqlx::query_as::<_, DbUser>(&query)
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
        let query = format!(
            "UPDATE users \
             SET email = COALESCE($2, email), \
                 password_hash = COALESCE($3, password_hash), \
                 name = COALESCE($4, name), \
                 is_active = COALESCE($5, is_active), \
                 is_verified = COALESCE($6, is_verified) \
             WHERE user_id = $1 \
             RETURNING {USER_SELECT_COLUMNS}"
        );

        let record = sqlx::query_as::<_, DbUser>(&query)
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
        let result = sqlx::query("DELETE FROM users WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_user(&self, user_id: UserId) -> Result<Option<DbUser>, DataError> {
        let query = format!("SELECT {USER_SELECT_COLUMNS} FROM users WHERE user_id = $1");

        let record = sqlx::query_as::<_, DbUser>(&query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(record)
    }

    async fn get_user_by_email(&self, email: String) -> Result<Option<DbUser>, DataError> {
        let query = format!("SELECT {USER_SELECT_COLUMNS} FROM users WHERE email = $1");

        let record = sqlx::query_as::<_, DbUser>(&query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(record)
    }

    async fn get_all_users(&self) -> Result<Vec<DbUser>, DataError> {
        let query = format!("SELECT {USER_SELECT_COLUMNS} FROM users ORDER BY user_id");

        let records = sqlx::query_as::<_, DbUser>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(records)
    }
}

use std::{future::Future, pin::Pin};

use domain::{create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};
use sqlx::PgPool;

use super::user_provider::UserProvider;

#[derive(sqlx::FromRow)]
struct UserRow {
    user_id: i64,
    email: String,
    name: String,
    picture: String,
    google_sub_id: Option<String>,
    is_admin: bool,
    is_active: bool,
    password_hash: Option<String>,
    is_verified: bool,
}

impl From<UserRow> for DomainUser {
    fn from(r: UserRow) -> Self {
        DomainUser {
            id: r.user_id,
            email: r.email,
            name: r.name,
            picture: r.picture,
            google_sub_id: r.google_sub_id,
            is_admin: r.is_admin,
            is_active: r.is_active,
            password_hash: r.password_hash,
            is_verified: r.is_verified,
        }
    }
}

pub struct PostgresUserProvider {
    pool: PgPool,
}

impl PostgresUserProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserProvider for PostgresUserProvider {
    fn get_user_by_id<'a>(
        &'a self,
        id: i64,
    ) -> Pin<Box<dyn Future<Output = Option<DomainUser>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query_as::<_, UserRow>(
                "SELECT user_id, email, name, picture, google_sub_id, is_admin, is_active, password_hash, is_verified FROM users WHERE user_id = $1",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
            .map(DomainUser::from)
        })
    }

    fn get_user_by_google_sub_id<'a>(
        &'a self,
        sub_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<DomainUser>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query_as::<_, UserRow>(
                "SELECT user_id, email, name, picture, google_sub_id, is_admin, is_active, password_hash, is_verified FROM users WHERE google_sub_id = $1",
            )
            .bind(sub_id)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
            .map(DomainUser::from)
        })
    }

    fn get_user_by_email<'a>(
        &'a self,
        email: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<DomainUser>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query_as::<_, UserRow>(
                "SELECT user_id, email, name, picture, google_sub_id, is_admin, is_active, password_hash, is_verified FROM users WHERE email = $1",
            )
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
            .map(DomainUser::from)
        })
    }

    fn create_domain_user<'a>(
        &'a self,
        p: CreateDomainUserParams,
    ) -> Pin<Box<dyn Future<Output = Result<DomainUser, String>> + Send + 'a>> {
        Box::pin(async move {
            let row = sqlx::query_as::<_, UserRow>(
                r#"
                INSERT INTO users (email, name, picture, google_sub_id, password_hash, is_admin, is_active, is_verified)
                VALUES ($1, $2, $3, $4, $5, false, true, false)
                ON CONFLICT (email) DO UPDATE
                    SET google_sub_id = COALESCE(EXCLUDED.google_sub_id, users.google_sub_id),
                        name = EXCLUDED.name,
                        picture = EXCLUDED.picture
                RETURNING user_id, email, name, picture, google_sub_id, is_admin, is_active, password_hash, is_verified
                "#,
            )
            .bind(&p.email)
            .bind(&p.name)
            .bind(&p.picture)
            .bind(&p.google_sub_id)
            .bind(&p.password_hash)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            Ok(DomainUser::from(row))
        })
    }

    fn update_user<'a>(
        &self,
        user: DomainUser,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            sqlx::query(
                "UPDATE users SET email=$1, name=$2, picture=$3, is_admin=$4, is_active=$5, password_hash=$6, is_verified=$7 WHERE user_id=$8",
            )
            .bind(&user.email)
            .bind(&user.name)
            .bind(&user.picture)
            .bind(user.is_admin)
            .bind(user.is_active)
            .bind(&user.password_hash)
            .bind(user.is_verified)
            .bind(user.id)
            .execute(&pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
        })
    }

    fn delete_user<'a>(
        &'a self,
        id: i64,
    ) -> Pin<Box<dyn Future<Output = Result<DomainUser, String>> + Send + 'a>> {
        Box::pin(async move {
            let row = sqlx::query_as::<_, UserRow>(
                "DELETE FROM users WHERE user_id = $1 RETURNING user_id, email, name, picture, google_sub_id, is_admin, is_active, password_hash, is_verified",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("User {} not found", id))?;
            Ok(DomainUser::from(row))
        })
    }

    fn list_users<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<DomainUser>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query_as::<_, UserRow>(
                "SELECT user_id, email, name, picture, google_sub_id, is_admin, is_active, password_hash, is_verified FROM users",
            )
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(DomainUser::from)
            .collect()
        })
    }
}

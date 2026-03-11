use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::{FromRow, PgPool};

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::members::data_provider::members_data_provider::MembersDataProvider;
use crate::members::org_member::OrgMember;
use crate::members::OrgMemberRole;

#[derive(FromRow)]
struct MemberRow {
    user_id: i64,
    org_id: i64,
    email: String,
    full_name: String,
    is_active: bool,
}

#[derive(FromRow)]
struct MemberRoleRow {
    user_id: i64,
    role_id: i64,
    role_name: String,
    is_system: bool,
    role_org_id: Option<i64>,
}

pub struct MembersDataProviderService {
    pool: PgPool,
}

impl MembersDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MembersDataProvider for MembersDataProviderService {
    async fn list_org_members(&self, org_id: OrganizationId) -> Result<Vec<OrgMember>, DataError> {
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;

        let member_rows = sqlx::query_as::<_, MemberRow>(
            "SELECT m.user_id, m.org_id, u.email, u.full_name, u.is_active \
             FROM memberships m \
             JOIN users u ON u.id = m.user_id \
             WHERE m.org_id = $1 \
             ORDER BY m.user_id",
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        let role_rows = sqlx::query_as::<_, MemberRoleRow>(
            "SELECT mr.user_id, r.id AS role_id, r.name AS role_name, r.is_system, r.org_id AS role_org_id \
             FROM membership_roles mr \
             JOIN roles r ON r.id = mr.role_id \
             WHERE mr.org_id = $1",
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        let mut roles_by_user: HashMap<UserId, Vec<OrgMemberRole>> = HashMap::new();
        for row in role_rows {
            roles_by_user
                .entry(row.user_id)
                .or_default()
                .push(OrgMemberRole {
                    id: row.role_id,
                    name: row.role_name,
                    is_system: row.is_system,
                    org_id: row.role_org_id,
                });
        }

        let members = member_rows
            .into_iter()
            .map(|row| OrgMember {
                user_id: row.user_id,
                org_id: row.org_id,
                email: row.email,
                full_name: row.full_name,
                is_active: row.is_active,
                roles: roles_by_user.remove(&row.user_id).unwrap_or_default(),
            })
            .collect();

        Ok(members)
    }
}

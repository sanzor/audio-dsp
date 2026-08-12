use domain::{
    db::{db_workspace::WorkspaceId, db_track::TrackId},
    region_set::region_set_subtree::RegionSetSubtree,
    tracks::track_subtree::TrackSubtree,
};
use sqlx::PgPool;

use super::workspace_data_provider::WorkspaceDataProvider;

pub struct PostgresWorkspaceDataProvider {
    pool: PgPool,
}

impl PostgresWorkspaceDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct TrackWorkspaceRow {
    track_id: TrackId,
    name: String,
    extension: String,
    length_seconds: f32,
    #[sqlx(json)]
    region_sets: Vec<RegionSetSubtree>,
}

#[async_trait::async_trait]
impl WorkspaceDataProvider for PostgresWorkspaceDataProvider {
    async fn get_workspace_tracks(&self, workspace_id: WorkspaceId) -> Result<Vec<TrackSubtree>, String> {
        let rows = sqlx::query_as::<_, TrackWorkspaceRow>(
            "SELECT track_id, name, extension, length_seconds, region_sets \
             FROM get_workspace_tracks($1)",
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|row| TrackSubtree {
                track_id: row.track_id,
                name: row.name,
                extension: row.extension,
                length_seconds: row.length_seconds,
                region_sets: row.region_sets,
            })
            .collect())
    }
}

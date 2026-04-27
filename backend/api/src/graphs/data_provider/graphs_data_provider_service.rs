use domain::{
    db::{
        db_graph::{DbGraph, GraphId},
        db_region::RegionId,
    },
    graphs::{
        add_graph_params::AddGraphParams,
        copy_graph_params::CopyGraphParams,
        delete_graph_params::DeleteGraphParams,
        edit_graph_params::EditGraphParams,
        save_graph_state_params::SaveGraphStateParams,
    },
};
use sqlx::PgPool;

use super::graphs_data_provider::GraphsDataProvider;

pub struct PostgresGraphsDataProvider {
    pool: PgPool,
}

impl PostgresGraphsDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl GraphsDataProvider for PostgresGraphsDataProvider {
    async fn create_graph(&self, params: AddGraphParams) -> Result<DbGraph, String> {
        sqlx::query_as::<_, DbGraph>(
            r#"
            INSERT INTO graphs (region_id, name)
            VALUES ($1, $2)
            RETURNING graph_id, region_id, name, graph_state, version, created_at, updated_at
            "#,
        )
        .bind(params.region_id)
        .bind(params.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn copy_graph(&self, params: CopyGraphParams) -> Result<DbGraph, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let copied_graph = sqlx::query_as::<_, DbGraph>(
            r#"
            INSERT INTO graphs (region_id, name, graph_state, version, updated_at)
            SELECT $2, $3, graph_state, 1, now()
            FROM graphs
            WHERE graph_id = $1
            RETURNING graph_id, region_id, name, graph_state, version, created_at, updated_at
            "#,
        )
        .bind(params.source_graph_id)
        .bind(params.destination_region_id)
        .bind(params.copy_name)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(copied_graph)
    }

    async fn edit_graph(&self, params: EditGraphParams) -> Result<DbGraph, String> {
        sqlx::query_as::<_, DbGraph>(
            r#"
            UPDATE graphs
            SET name = COALESCE($2, name),
                updated_at = now()
            WHERE graph_id = $1
            RETURNING graph_id, region_id, name, graph_state, version, created_at, updated_at
            "#,
        )
        .bind(params.graph_id)
        .bind(params.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn save_graph_state(&self, params: SaveGraphStateParams) -> Result<DbGraph, String> {
        sqlx::query_as::<_, DbGraph>(
            r#"
            UPDATE graphs
            SET graph_state = $2,
                version = version + 1,
                updated_at = now()
            WHERE graph_id = $1
            RETURNING graph_id, region_id, name, graph_state, version, created_at, updated_at
            "#,
        )
        .bind(params.graph_id)
        .bind(params.state)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_graph(&self, params: DeleteGraphParams) -> Result<(), String> {
        sqlx::query(r#"DELETE FROM graphs WHERE graph_id = $1"#)
            .bind(params.graph_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn get_graph(&self, graph_id: &GraphId) -> Result<DbGraph, String> {
        sqlx::query_as::<_, DbGraph>(
            r#"
            SELECT graph_id, region_id, name, graph_state, version, created_at, updated_at
            FROM graphs
            WHERE graph_id = $1
            "#,
        )
        .bind(graph_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_graph_for_region(
        &self,
        region_id: &RegionId,
    ) -> Result<Option<DbGraph>, String> {
        sqlx::query_as::<_, DbGraph>(
            r#"
            SELECT graph_id, region_id, name, graph_state, version, created_at, updated_at
            FROM graphs
            WHERE region_id = $1
            "#,
        )
        .bind(region_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

use domain::{
    db::{
        db_edge::DbEdge,
        db_graph::{DbGraph, GraphId},
        db_node::DbNode,
        db_region::RegionId,
    },
    graphs::{
        add_graph_params::AddGraphParams,
        delete_graph_params::DeleteGraphParams,
        edit_graph_params::EditGraphParams,
        edge::Edge,
        graph_subtree::GraphSubtree,
        node::Node,
    },
};
use sqlx::PgPool;
use ulid::Ulid;

use super::graphs_provider::GraphsProvider;

pub struct PostgresGraphsProvider {
    pool: PgPool,
}

impl PostgresGraphsProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl GraphsProvider for PostgresGraphsProvider {
    async fn create_graph(&self, params: AddGraphParams) -> Result<DbGraph, String> {
        let graph_id: GraphId = Ulid::new().to_string();

        sqlx::query_as::<_, DbGraph>(
            r#"
            INSERT INTO graphs (graph_id, region_id, name)
            VALUES ($1, $2, $3)
            RETURNING graph_id, region_id, name, created_at
            "#,
        )
        .bind(graph_id)
        .bind(params.region_id)
        .bind(params.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn edit_graph(&self, params: EditGraphParams) -> Result<DbGraph, String> {
        sqlx::query_as::<_, DbGraph>(
            r#"
            UPDATE graphs
            SET name = COALESCE($2, name)
            WHERE graph_id = $1
            RETURNING graph_id, region_id, name, created_at
            "#,
        )
        .bind(params.graph_id)
        .bind(params.name)
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
            SELECT graph_id, region_id, name, created_at
            FROM graphs
            WHERE graph_id = $1
            "#,
        )
        .bind(graph_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_graph_for_region(&self, region_id: &RegionId) -> Result<Option<DbGraph>, String> {
        sqlx::query_as::<_, DbGraph>(
            r#"
            SELECT graph_id, region_id, name, created_at
            FROM graphs
            WHERE region_id = $1
            "#,
        )
        .bind(region_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_nodes(&self, graph_id: &GraphId) -> Result<Vec<DbNode>, String> {
        sqlx::query_as::<_, DbNode>(
            r#"
            SELECT node_id, graph_id, created_at
            FROM graph_nodes
            WHERE graph_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(graph_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_edges(&self, graph_id: &GraphId) -> Result<Vec<DbEdge>, String> {
        sqlx::query_as::<_, DbEdge>(
            r#"
            SELECT edge_id, graph_id, created_at
            FROM graph_edges
            WHERE graph_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(graph_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_graph_subtree(&self, graph_id: &GraphId) -> Result<GraphSubtree, String> {
        let graph = self.get_graph(graph_id).await?;
        let db_nodes = self.get_nodes(graph_id).await?;
        let db_edges = self.get_edges(graph_id).await?;

        Ok(GraphSubtree {
            graph_id: graph.graph_id,
            region_id: graph.region_id,
            name: graph.name,
            nodes: db_nodes
                .into_iter()
                .map(|n| Node {
                    id: n.node_id,
                    graph_id: n.graph_id,
                })
                .collect(),
            edges: db_edges
                .into_iter()
                .map(|e| Edge {
                    id: e.edge_id,
                    graph_id: e.graph_id,
                })
                .collect(),
        })
    }
}

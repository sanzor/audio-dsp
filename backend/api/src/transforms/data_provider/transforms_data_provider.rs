use domain::db::db_transform::{DbTransform, DbTransformPort, TransformId};

#[async_trait::async_trait]
pub trait TransformsDataProvider: Send + Sync {
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String>;
    async fn get_transforms_paginated(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String>;
    async fn get_ports_for_transform(&self, id: TransformId) -> Result<Vec<DbTransformPort>, String>;
    async fn insert_transform(&self, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransform, String>;
    async fn update_transform(&self, id: TransformId, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransform, String>;
    async fn delete_transform(&self, id: TransformId) -> Result<(), String>;
    async fn insert_port(&self, transform_id: TransformId, name: String, direction: String, port_order: i32, description: Option<String>) -> Result<DbTransformPort, String>;
    async fn delete_port(&self, port_id: i64) -> Result<(), String>;
}

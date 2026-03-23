use domain::{
    db::db_transform::TransformId,
    transforms::{transform::Transform, transform_port::TransformPort},
};

#[async_trait::async_trait]
pub trait TransformsProvider: Send + Sync {
    async fn get_transform(&self, id: TransformId) -> Result<Transform, String>;
    async fn get_transforms_paginated(&self, offset: i64, limit: i64) -> Result<(Vec<Transform>, i64), String>;
    async fn create_transform(&self, name: String, description: Option<String>, icon: Option<String>) -> Result<Transform, String>;
    async fn update_transform(&self, id: TransformId, name: String, description: Option<String>, icon: Option<String>) -> Result<Transform, String>;
    async fn delete_transform(&self, id: TransformId) -> Result<(), String>;
    async fn add_port(&self, transform_id: TransformId, name: String, direction: String, port_order: i32, description: Option<String>) -> Result<TransformPort, String>;
    async fn delete_port(&self, port_id: i64) -> Result<(), String>;
}

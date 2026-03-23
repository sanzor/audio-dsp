use std::sync::Arc;

use domain::{
    db::db_transform::{DbTransform, DbTransformPort, TransformId},
    transforms::{transform::Transform, transform_port::TransformPort},
};

use super::{
    data_provider::transforms_data_provider::TransformsDataProvider,
    transforms_provider::TransformsProvider,
};

pub struct TransformsProviderService {
    data: Arc<dyn TransformsDataProvider>,
}

impl TransformsProviderService {
    pub fn new(data: Arc<dyn TransformsDataProvider>) -> Self {
        Self { data }
    }

    fn port_to_domain(p: DbTransformPort) -> TransformPort {
        TransformPort {
            port_id: p.port_id,
            name: p.name,
            direction: p.direction,
            port_order: p.port_order,
            description: p.description,
        }
    }

    async fn with_ports(&self, db: DbTransform) -> Result<Transform, String> {
        let ports = self.data.get_ports_for_transform(db.transform_id).await?;
        Ok(Transform {
            transform_id: db.transform_id,
            name: db.name,
            description: db.description,
            icon: db.icon,
            ports: ports.into_iter().map(Self::port_to_domain).collect(),
        })
    }
}

#[async_trait::async_trait]
impl TransformsProvider for TransformsProviderService {
    async fn get_transform(&self, id: TransformId) -> Result<Transform, String> {
        let db = self.data.get_transform(id).await?;
        self.with_ports(db).await
    }

    async fn get_transforms_paginated(&self, offset: i64, limit: i64) -> Result<(Vec<Transform>, i64), String> {
        let (dbs, total) = self.data.get_transforms_paginated(offset, limit).await?;
        let mut result = Vec::with_capacity(dbs.len());
        for db in dbs {
            result.push(self.with_ports(db).await?);
        }
        Ok((result, total))
    }

    async fn create_transform(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
    ) -> Result<Transform, String> {
        let db = self.data.insert_transform(name, description, icon).await?;
        self.with_ports(db).await
    }

    async fn update_transform(
        &self,
        id: TransformId,
        name: String,
        description: Option<String>,
        icon: Option<String>,
    ) -> Result<Transform, String> {
        let db = self.data.update_transform(id, name, description, icon).await?;
        self.with_ports(db).await
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), String> {
        self.data.delete_transform(id).await
    }

    async fn add_port(
        &self,
        transform_id: TransformId,
        name: String,
        direction: String,
        port_order: i32,
        description: Option<String>,
    ) -> Result<TransformPort, String> {
        let db = self.data.insert_port(transform_id, name, direction, port_order, description).await?;
        Ok(Self::port_to_domain(db))
    }

    async fn delete_port(&self, port_id: i64) -> Result<(), String> {
        self.data.delete_port(port_id).await
    }
}

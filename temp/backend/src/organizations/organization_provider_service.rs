use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, warn};

use crate::billing_mode::billing_mode::BillingMode;
use crate::billing_mode::billing_mode_provider::BillingModeProvider;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::organizations::create_organization_params::CreateOrganizationParams;
use crate::organizations::create_organization_result::CreateOrganizationWithOwnerResult;
use crate::organizations::data_provider::organization_data_provider::OrganizationDataProvider;
use crate::organizations::update_organization_params::UpdateOrganizationParams;
use crate::organizations::DbOrganization;
use crate::organizations::{
    organization_provider::OrganizationProvider, CreateOrganizationResult, GetOrganizationResult,
    UpdateOrganizationResult,
};
use crate::rate_limits_config::rate_limit_config_input::RateLimitConfigInput;
use crate::rate_limits_config::rate_limits_config_provider::RateLimitsConfigProvider;
use crate::rate_limits_config::RateLimitRefillInput;
use crate::stripe::rate_limit_service::rate_limit_service::RateLimitService;

pub struct OrganizationProviderService {
    data_provider: Arc<dyn OrganizationDataProvider>,
    billing_mode_provider: Arc<dyn BillingModeProvider>,
    rate_limits_config_provider: Arc<dyn RateLimitsConfigProvider>,
    rate_limit_service: Arc<dyn RateLimitService>,
    default_token_bucket_size: u64,
}

impl OrganizationProviderService {
    pub fn new(
        data_provider: Arc<dyn OrganizationDataProvider>,
        billing_mode_provider: Arc<dyn BillingModeProvider>,
        rate_limits_config_provider: Arc<dyn RateLimitsConfigProvider>,
        rate_limit_service: Arc<dyn RateLimitService>,
        default_token_bucket_size: u64,
    ) -> Self {
        Self {
            data_provider,
            billing_mode_provider,
            rate_limits_config_provider,
            rate_limit_service,
            default_token_bucket_size,
        }
    }

    async fn init_org_defaults(&self, org_id: OrganizationId) {
        let size = self.default_token_bucket_size;

        if let Err(e) = self
            .billing_mode_provider
            .upsert_mode(org_id, BillingMode::TokenBucket)
            .await
        {
            warn!(org_id, error = %e, "failed to set default billing mode");
        }

        if let Err(e) = self
            .rate_limits_config_provider
            .upsert_config(org_id, RateLimitConfigInput::TokenBucket { limit: size })
            .await
        {
            warn!(org_id, error = %e, "failed to set default rate limit config");
        }

        if let Err(e) = self
            .rate_limit_service
            .apply_refill(org_id, RateLimitRefillInput::BucketAdd { amount: size })
            .await
        {
            warn!(org_id, error = %e, "failed to set initial token bucket");
        }
    }
}

#[async_trait]
impl OrganizationProvider for OrganizationProviderService {
    async fn create_organization(
        &self,
        params: CreateOrganizationParams,
    ) -> Result<CreateOrganizationResult, ServiceError> {
        info!(name = %params.name, "create organization requested");
        let record = self.data_provider.create_organization(params).await?;
        self.init_org_defaults(record.id).await;
        Ok(CreateOrganizationResult {
            organization: record,
        })
    }

    async fn create_organization_with_owner(
        &self,
        params: CreateOrganizationParams,
        user_id: UserId,
    ) -> Result<CreateOrganizationWithOwnerResult, ServiceError> {
        info!(name = %params.name, user_id, "create organization with owner requested");
        let result = self
            .data_provider
            .create_organization_with_owner(params, user_id)
            .await?;
        self.init_org_defaults(result.organization.id).await;
        Ok(result)
    }

    async fn update_organization(
        &self,
        org_id: OrganizationId,
        params: UpdateOrganizationParams,
    ) -> Result<Option<UpdateOrganizationResult>, ServiceError> {
        info!(org_id, name = ?params.name, "update organization requested");
        let record = self
            .data_provider
            .update_organization(org_id, params)
            .await?;
        let Some(updated) = record else {
            return Ok(None);
        };

        Ok(Some(UpdateOrganizationResult {
            organization: updated,
        }))
    }

    async fn delete_organization(&self, org_id: OrganizationId) -> Result<bool, ServiceError> {
        info!(org_id, "delete organization requested");
        Ok(self.data_provider.delete_organization(org_id).await?)
    }

    async fn get_organization(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<GetOrganizationResult>, ServiceError> {
        info!(org_id, "get organization requested");
        let record = self.data_provider.get_organization(org_id).await?;
        Ok(record.map(|organization| GetOrganizationResult { organization }))
    }

    async fn get_all_organizations(&self) -> Result<Vec<DbOrganization>, ServiceError> {
        info!("get all organizations requested");
        Ok(self.data_provider.get_all_organizations().await?)
    }
}

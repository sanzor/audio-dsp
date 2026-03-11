use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, warn};

use crate::checkout::checkout_error::CheckoutError;
use crate::checkout::checkout_provider::CheckoutProvider;
use crate::checkout::create_subscription_checkout_params::CreateSubscriptionCheckoutParams;
use crate::checkout::create_subscription_checkout_result::CreateSubscriptionCheckoutResult;
use crate::checkout::create_token_pack_checkout_params::CreateTokenPackCheckoutParams;
use crate::checkout::create_token_pack_checkout_result::CreateTokenPackCheckoutResult;
use crate::domain::db::db_product::ProductId;
use crate::organizations::organization_provider::OrganizationProvider;
use crate::organizations::update_organization_params::UpdateOrganizationParams;
use crate::products::products_provider::ProductsProvider;
use crate::stripe::stripe_service::StripeService;
use crate::tier_configs::tier_configs_provider::TierConfigsProvider;

pub struct CheckoutProviderService {
    org_provider: Arc<dyn OrganizationProvider>,
    products_provider: Arc<dyn ProductsProvider>,
    tier_configs_provider: Arc<dyn TierConfigsProvider>,
    stripe: Arc<StripeService>,
}

impl CheckoutProviderService {
    pub fn new(
        org_provider: Arc<dyn OrganizationProvider>,
        products_provider: Arc<dyn ProductsProvider>,
        tier_configs_provider: Arc<dyn TierConfigsProvider>,
        stripe: Arc<StripeService>,
    ) -> Self {
        Self {
            org_provider,
            products_provider,
            tier_configs_provider,
            stripe,
        }
    }

    /// Retrieve or lazily create a Stripe Customer for the org.
    /// Saves the customer ID back to the DB on first creation.
    async fn ensure_stripe_customer(
        &self,
        org_id: crate::domain::db::db_organization::OrganizationId,
    ) -> Result<String, CheckoutError> {
        let org = self
            .org_provider
            .get_organization(org_id)
            .await
            .map_err(|e| CheckoutError::Internal(e.to_string()))?
            .ok_or_else(|| CheckoutError::NotFound("organization not found".to_string()))?
            .organization;

        if let Some(id) = org.stripe_customer_id.clone() {
            match self.stripe.retrieve_customer(&id).await {
                Ok(_) => return Ok(id),
                Err(msg) if msg.contains("No such customer") => {
                    warn!(
                        org_id = %org_id,
                        stripe_customer_id = %id,
                        "stored stripe_customer_id not found in Stripe; recreating"
                    );
                }
                Err(msg) => {
                    return Err(CheckoutError::Internal(format!(
                        "failed to retrieve stripe customer: {msg}"
                    )));
                }
            }
        }

        let stripe_email = looks_like_stripe_email(&org.billing_email)
            .then(|| org.billing_email.trim())
            .filter(|s| !s.is_empty());

        let customer = self
            .stripe
            .create_customer(stripe_email, &org.name)
            .await
            .map_err(|e| {
                error!(error = %e, "failed to create stripe customer");
                CheckoutError::Internal(e)
            })?;

        let customer_id = customer.id.to_string();

        self.org_provider
            .update_organization(
                org_id,
                UpdateOrganizationParams {
                    name: None,
                    slug: None,
                    billing_email: None,
                    stripe_customer_id: Some(customer_id.clone()),
                    status: None,
                },
            )
            .await
            .map_err(|e| CheckoutError::Internal(e.to_string()))?;

        Ok(customer_id)
    }
}

fn looks_like_stripe_email(email: &str) -> bool {
    let trimmed = email.trim();
    if trimmed.is_empty() || trimmed.contains(char::is_whitespace) {
        return false;
    }
    let Some((_, domain)) = trimmed.split_once('@') else {
        return false;
    };
    domain.contains('.')
}

#[async_trait]
impl CheckoutProvider for CheckoutProviderService {
    async fn create_token_pack_checkout(
        &self,
        params: CreateTokenPackCheckoutParams,
    ) -> Result<CreateTokenPackCheckoutResult, CheckoutError> {
        let pid = ProductId::try_from(params.product_id)
            .map_err(|_| CheckoutError::InvalidInput("invalid product_id".to_string()))?;

        let product = self
            .products_provider
            .get_product(pid)
            .await
            .map_err(|e| CheckoutError::Internal(e.to_string()))?
            .ok_or_else(|| CheckoutError::NotFound("product not found".to_string()))?;

        let stripe_price_id = product.stripe_price_id.clone().ok_or_else(|| {
            CheckoutError::InvalidInput("product has no stripe_price_id".to_string())
        })?;

        let customer_id = self.ensure_stripe_customer(params.org_id).await?;

        let default_success_path = format!("/org/{}/checkout/success", params.org_slug);
        let default_cancel_path = format!("/org/{}/checkout/cancel", params.org_slug);
        let success_target = params
            .success_url
            .as_deref()
            .unwrap_or(&default_success_path);
        let cancel_target = params
            .cancel_url
            .as_deref()
            .unwrap_or(&default_cancel_path);

        let session = self
            .stripe
            .create_token_pack_session(
                &customer_id,
                &stripe_price_id,
                i64::from(params.org_id),
                i64::from(product.id),
                product.token_amount,
                success_target,
                cancel_target,
            )
            .await
            .map_err(|e| CheckoutError::Internal(e))?;

        Ok(CreateTokenPackCheckoutResult {
            checkout_url: session.url.unwrap_or_default(),
        })
    }

    async fn create_subscription_checkout(
        &self,
        params: CreateSubscriptionCheckoutParams,
    ) -> Result<CreateSubscriptionCheckoutResult, CheckoutError> {
        let tier_config = self
            .tier_configs_provider
            .get(&params.tier.to_string())
            .await
            .map_err(|e| CheckoutError::Internal(e.to_string()))?
            .ok_or_else(|| CheckoutError::NotFound("tier config not found".to_string()))?;

        let stripe_price_id = tier_config.stripe_price_id.clone().ok_or_else(|| {
            CheckoutError::InvalidInput("tier has no stripe_price_id configured".to_string())
        })?;

        let customer_id = self.ensure_stripe_customer(params.org_id).await?;

        let default_success_path = format!("/org/{}/checkout/success", params.org_slug);
        let default_cancel_path = format!("/org/{}/checkout/cancel", params.org_slug);
        let success_target = params
            .success_url
            .as_deref()
            .unwrap_or(&default_success_path);
        let cancel_target = params
            .cancel_url
            .as_deref()
            .unwrap_or(&default_cancel_path);

        let session = self
            .stripe
            .create_subscription_session(
                &customer_id,
                &stripe_price_id,
                i64::from(params.org_id),
                &params.tier.to_string(),
                success_target,
                cancel_target,
            )
            .await
            .map_err(|e| CheckoutError::Internal(e))?;

        Ok(CreateSubscriptionCheckoutResult {
            checkout_url: session.url.unwrap_or_default(),
        })
    }
}

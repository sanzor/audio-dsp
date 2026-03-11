use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, info, warn};

use crate::domain::db::db_product::ProductId;
use crate::domain::service_error::ServiceError;
use crate::products::create_product_params::CreateProductParams;
use crate::products::data_provider::products_data_provider::ProductsDataProvider;
use crate::products::products_provider::ProductsProvider;
use crate::products::update_product_params::UpdateProductParams;
use crate::products::DbProduct;
use crate::stripe::stripe_service::StripeService;

pub struct ProductsProviderService {
    data_provider: Arc<dyn ProductsDataProvider>,
    stripe_service: Arc<StripeService>,
}

impl ProductsProviderService {
    pub fn new(
        data_provider: Arc<dyn ProductsDataProvider>,
        stripe_service: Arc<StripeService>,
    ) -> Self {
        Self {
            data_provider,
            stripe_service,
        }
    }
}

#[async_trait]
impl ProductsProvider for ProductsProviderService {
    async fn create_product(&self, params: CreateProductParams) -> Result<DbProduct, ServiceError> {
        info!(name = %params.name, "create product requested");

        // Auto-create a Stripe Product + Price if no price ID was provided.
        let (stripe_price_id, stripe_product_id) = match params.stripe_price_id {
            Some(id) => (Some(id), None),
            None => {
                // Step 1: create the Stripe Product so we have its ID for activate/deactivate.
                let stripe_product = self
                    .stripe_service
                    .create_product(&params.name, params.description.as_deref())
                    .await
                    .map_err(|e| {
                        error!(error = %e, "failed to create Stripe product");
                        ServiceError::from(e)
                    })?;
                let stripe_product_id = stripe_product.id.to_string();

                // Step 2: create a one-time Price attached to that product.
                let price = self
                    .stripe_service
                    .create_price_for_existing_product(
                        &stripe_product_id,
                        params.price_cents,
                        &params.currency,
                    )
                    .await
                    .map_err(|e| {
                        error!(error = %e, "failed to create Stripe price for product");
                        ServiceError::from(e)
                    })?;

                (Some(price.id.to_string()), Some(stripe_product_id))
            }
        };

        self.data_provider
            .create_product(
                params.name,
                params.description,
                params.token_amount,
                params.price_cents,
                params.currency,
                stripe_price_id,
                stripe_product_id,
            )
            .await
            .map_err(ServiceError::from)
    }

    async fn update_product(
        &self,
        id: ProductId,
        params: UpdateProductParams,
    ) -> Result<Option<DbProduct>, ServiceError> {
        info!(id, "update product requested");

        // If price is changing, rotate the Stripe price.
        let stripe_price_id = if params.price_cents.is_some() {
            match self.data_provider.get_product(id).await? {
                Some(current) => {
                    if let Some(ref old_price_id) = current.stripe_price_id {
                        if let Err(e) = self.stripe_service.archive_price(old_price_id).await {
                            error!(id, error = %e, "failed to archive old Stripe price");
                            return Err(ServiceError::from(e));
                        }
                    }

                    let name = params.name.as_deref().unwrap_or(&current.name);
                    let price_cents = params.price_cents.unwrap();
                    let currency = &current.currency;

                    let new_price = if let Some(ref prod_id) = current.stripe_product_id {
                        self.stripe_service
                            .create_price_for_existing_product(prod_id, price_cents, currency)
                            .await
                    } else {
                        self.stripe_service
                            .create_one_time_price(name, price_cents, currency)
                            .await
                    };

                    match new_price {
                        Ok(price) => Some(Some(price.id.to_string())),
                        Err(e) => {
                            error!(id, error = %e, "failed to create new Stripe price");
                            return Err(ServiceError::from(e));
                        }
                    }
                }
                None => {
                    warn!(id, "update_product: product not found");
                    None
                }
            }
        } else {
            None
        };

        self.data_provider
            .update_product(
                id,
                params.name,
                params.description,
                params.token_amount,
                params.price_cents,
                stripe_price_id.unwrap_or(params.stripe_price_id),
                params.is_active,
            )
            .await
            .map_err(ServiceError::from)
    }

    async fn activate_product(&self, id: ProductId) -> Result<(), ServiceError> {
        info!(id, "activate product requested");

        let product = self
            .data_provider
            .get_product(id)
            .await?
            .ok_or(ServiceError::NotFound)?;

        if product.stripe_price_id.is_none() {
            return Err(ServiceError::Validation(
                "product has no stripe_price_id; assign a Stripe price before activating"
                    .to_string(),
            ));
        }

        if let Some(ref stripe_product_id) = product.stripe_product_id {
            if let Err(e) = self
                .stripe_service
                .activate_product(stripe_product_id)
                .await
            {
                error!(id, error = %e, "failed to activate Stripe product");
                return Err(ServiceError::from(e));
            }
        } else {
            warn!(
                id,
                "activate_product: no stripe_product_id, skipping Stripe call"
            );
        }

        self.data_provider
            .update_product(id, None, None, None, None, None, Some(true))
            .await?;

        Ok(())
    }

    async fn deactivate_product(&self, id: ProductId) -> Result<(), ServiceError> {
        info!(id, "deactivate product requested");

        let product = self
            .data_provider
            .get_product(id)
            .await?
            .ok_or(ServiceError::NotFound)?;

        if let Some(ref stripe_product_id) = product.stripe_product_id {
            if let Err(e) = self
                .stripe_service
                .deactivate_product(stripe_product_id)
                .await
            {
                error!(id, error = %e, "failed to deactivate Stripe product");
                return Err(ServiceError::from(e));
            }
        } else {
            warn!(
                id,
                "deactivate_product: no stripe_product_id, skipping Stripe call"
            );
        }

        self.data_provider
            .update_product(id, None, None, None, None, None, Some(false))
            .await?;

        Ok(())
    }

    async fn delete_product(&self, id: ProductId) -> Result<bool, ServiceError> {
        info!(id, "delete product requested");

        let product = self
            .data_provider
            .get_product(id)
            .await?
            .ok_or(ServiceError::NotFound)?;

        if let Some(ref price_id) = product.stripe_price_id {
            match self.stripe_service.archive_price(price_id).await {
                Ok(_) => {}
                Err(e) if stripe_not_found(&e) => {
                    warn!(id, price_id, "Stripe price not found during delete; skipping archive");
                }
                Err(e) => {
                    error!(id, error = %e, "failed to archive Stripe price on product delete");
                    return Err(ServiceError::from(e));
                }
            }
        }

        if let Some(ref stripe_product_id) = product.stripe_product_id {
            match self.stripe_service.delete_product(stripe_product_id).await {
                Ok(_) => {}
                Err(e) if stripe_not_found(&e) => {
                    warn!(id, stripe_product_id, "Stripe product not found during delete; skipping");
                }
                Err(e) => {
                    error!(id, error = %e, "failed to delete Stripe product");
                    return Err(ServiceError::from(e));
                }
            }
        }

        self.data_provider
            .delete_product(id)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_product(&self, id: ProductId) -> Result<Option<DbProduct>, ServiceError> {
        info!(id, "get product requested");
        self.data_provider
            .get_product(id)
            .await
            .map_err(ServiceError::from)
    }

    async fn list_products(&self) -> Result<Vec<DbProduct>, ServiceError> {
        info!("list products requested");
        self.data_provider
            .list_products()
            .await
            .map_err(ServiceError::from)
    }
}

/// Returns true when a Stripe error means the object simply doesn't exist
/// (e.g. deleted, or belongs to the other live/test environment).
fn stripe_not_found(msg: &str) -> bool {
    msg.contains("No such price")
        || msg.contains("No such product")
        || msg.contains("No such customer")
}

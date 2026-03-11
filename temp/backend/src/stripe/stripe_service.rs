use std::collections::HashMap;

use rust_shared::organization_id::OrganizationId;
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionPaymentMethodTypes,
    CreateCheckoutSessionSubscriptionData, CreateCustomer, CreateProduct, Currency, Customer,
    CustomerId, Event, Price, Product, UpdatePrice, Webhook,
};
use tracing::{error, warn};

use crate::domain::db::db_product::ProductId;

pub struct StripeService {
    client: Client,
    webhook_secret: String,
    portal_url: String,
}

impl StripeService {
    fn ensure_valid_portal_url(portal_url: &str) -> Result<(), String> {
        let u = portal_url.trim();
        if u.is_empty() {
            return Err("stripe.portal_url is not set (expected frontend base URL like \"http://localhost:5173\")"
                .to_string());
        }
        if !(u.starts_with("http://") || u.starts_with("https://")) {
            return Err(format!(
                "stripe.portal_url must be an absolute http(s) URL (got: \"{u}\")"
            ));
        }
        Ok(())
    }
    pub fn new(secret_key: &str, webhook_secret: String, portal_url: String) -> Self {
        Self {
            client: Client::new(secret_key),
            webhook_secret,
            portal_url: portal_url.trim_end_matches('/').to_string(),
        }
    }

    // ── Customer ──────────────────────────────────────────────────────────────

    pub async fn create_customer(&self, email: Option<&str>, name: &str) -> Result<Customer, String> {
        let mut params = CreateCustomer::new();
        if let Some(email) = email {
            if looks_like_stripe_email(email) {
                params.email = Some(email);
            } else {
                warn!(email = %email, "skipping invalid stripe customer email");
            }
        }
        params.name = Some(name);
        Customer::create(&self.client, params).await.map_err(|e| {
            error!(error = %e, "stripe create_customer failed");
            e.to_string()
        })
    }

    pub async fn retrieve_customer(&self, stripe_customer_id: &str) -> Result<Customer, String> {
        let id = stripe_customer_id
            .parse::<CustomerId>()
            .map_err(|e| e.to_string())?;
        Customer::retrieve(&self.client, &id, &[])
            .await
            .map_err(|e| {
                error!(error = %e, stripe_customer_id, "stripe retrieve_customer failed");
                e.to_string()
            })
    }
    pub async fn create_product(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<Product, String> {
        let mut params = CreateProduct::new(name);
        params.description = description;
        params.active = Some(true);
        Product::create(&self.client, params).await.map_err(|e| {
            error!(error = %e, "stripe create_product failed");
            e.to_string()
        })
    }

    // ── Checkout Session — one-time token pack ────────────────────────────────

    pub async fn create_token_pack_session(
        &self,
        stripe_customer_id: &str,
        stripe_price_id: &str,
        org_id: OrganizationId,
        product_id: ProductId,
        tokens_granted: i64,
        success_target: &str,
        cancel_target: &str,
    ) -> Result<CheckoutSession, String> {
        let mut params = CreateCheckoutSession::new();
        let success_url = self.resolve_redirect_target(success_target)?;
        let cancel_url = self.resolve_redirect_target(cancel_target)?;
        params.mode = Some(CheckoutSessionMode::Payment);
        // Avoid relying on Dashboard "dynamic payment methods" being configured.
        // If no compatible payment methods are enabled for the price currency, Stripe returns:
        // "No valid payment method types for this Checkout Session..."
        params.payment_method_types = Some(vec![CreateCheckoutSessionPaymentMethodTypes::Card]);
        params.customer = Some(
            stripe_customer_id
                .parse::<CustomerId>()
                .map_err(|e| e.to_string())?,
        );
        params.success_url = Some(&success_url);
        params.cancel_url = Some(&cancel_url);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            price: Some(stripe_price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);
        let mut metadata = HashMap::new();
        metadata.insert("org_id".to_string(), org_id.to_string());
        metadata.insert("product_id".to_string(), product_id.to_string());
        metadata.insert("tokens_granted".to_string(), tokens_granted.to_string());
        params.metadata = Some(metadata);

        CheckoutSession::create(&self.client, params)
            .await
            .map_err(|e| {
                error!(error = %e, "stripe create_token_pack_session failed");
                e.to_string()
            })
    }

    // ── Checkout Session — subscription ───────────────────────────────────────

    pub async fn create_subscription_session(
        &self,
        stripe_customer_id: &str,
        stripe_price_id: &str,
        org_id: i64,
        tier: &str,
        success_target: &str,
        cancel_target: &str,
    ) -> Result<CheckoutSession, String> {
        // Embed org_id + tier in both session metadata AND subscription metadata.
        // Session metadata is used in `checkout.session.completed`.
        // Subscription metadata is used in lifecycle events (subscription.updated/deleted).
        let mut meta = HashMap::new();
        meta.insert("org_id".to_string(), org_id.to_string());
        meta.insert("tier".to_string(), tier.to_string());

        let mut sub_data = CreateCheckoutSessionSubscriptionData::default();
        sub_data.metadata = Some(meta.clone());

        let success_url = self.resolve_redirect_target(success_target)?;
        let cancel_url = self.resolve_redirect_target(cancel_target)?;
        let mut params = CreateCheckoutSession::new();
        params.mode = Some(CheckoutSessionMode::Subscription);
        // Same rationale as `create_token_pack_session`.
        params.payment_method_types = Some(vec![CreateCheckoutSessionPaymentMethodTypes::Card]);
        params.customer = Some(
            stripe_customer_id
                .parse::<CustomerId>()
                .map_err(|e| e.to_string())?,
        );
        params.success_url = Some(&success_url);
        params.cancel_url = Some(&cancel_url);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            price: Some(stripe_price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);
        params.metadata = Some(meta);
        params.subscription_data = Some(sub_data);

        CheckoutSession::create(&self.client, params)
            .await
            .map_err(|e| {
                error!(error = %e, "stripe create_subscription_session failed");
                e.to_string()
            })
    }

    // ── Price (for token-pack products) ──────────────────────────────────────

    pub async fn create_one_time_price(
        &self,
        product_name: &str,
        unit_amount: i64,
        currency: &str,
    ) -> Result<Price, String> {
        use stripe::{CreatePrice, CreatePriceProductData};

        let currency: Currency = currency.parse().unwrap_or(Currency::USD);
        let mut params = CreatePrice::new(currency);
        params.unit_amount = Some(unit_amount);
        // Stripe's Price create API expects either `product` (an existing product ID)
        // or `product_data` (to create a product inline). Passing an object via
        // `product` can be interpreted as an invalid string by Stripe.
        params.product_data = Some(CreatePriceProductData {
            name: product_name.to_string(),
            ..Default::default()
        });

        Price::create(&self.client, params).await.map_err(|e| {
            error!(error = %e, "stripe create_one_time_price failed");
            e.to_string()
        })
    }

    pub async fn archive_price(&self, price_id: &str) -> Result<Price, String> {
        use stripe::{PriceId, Product, ProductId, UpdateProduct};
        let id: PriceId = price_id
            .parse()
            .map_err(|e: stripe::ParseIdError| e.to_string())?;

        let mut params = UpdatePrice::new();
        params.active = Some(false);

        match Price::update(&self.client, &id, params.clone()).await {
            Ok(p) => Ok(p),
            Err(e) => {
                let msg = e.to_string();

                // Stripe will reject archiving a price that is still set as its product's
                // `default_price`. Clear the product default price and retry once.
                if msg.contains("default price of its product") {
                    match Price::retrieve(&self.client, &id, &[]).await {
                        Ok(price) => {
                            let product_id: Option<ProductId> =
                                price.product.as_ref().map(|p| p.id());

                            if let Some(prod_id) = product_id {
                                let mut update = UpdateProduct::new();
                                // Stripe allows clearing fields by posting an empty string.
                                update.default_price = Some("");
                                if let Err(e2) =
                                    Product::update(&self.client, &prod_id, update).await
                                {
                                    error!(error = %e2, price_id, "stripe clear product default_price failed");
                                    return Err(e2.to_string());
                                }

                                return Price::update(&self.client, &id, params).await.map_err(|e3| {
                                    error!(error = %e3, price_id, "stripe archive_price retry failed");
                                    e3.to_string()
                                });
                            }
                        }
                        Err(e2) => {
                            error!(error = %e2, price_id, "stripe retrieve price failed while archiving");
                            return Err(e2.to_string());
                        }
                    }
                }

                error!(error = %e, price_id, "stripe archive_price failed");
                Err(msg)
            }
        }
    }

    // ── Subscription management ───────────────────────────────────────────────

    /// Cancel a Stripe subscription immediately.
    pub async fn cancel_subscription(&self, stripe_sub_id: &str) -> Result<(), String> {
        use stripe::{CancelSubscription, Subscription, SubscriptionId};
        let id: SubscriptionId = stripe_sub_id
            .parse()
            .map_err(|e: stripe::ParseIdError| e.to_string())?;
        Subscription::cancel(&self.client, &id, CancelSubscription::default())
            .await
            .map_err(|e| {
                error!(error = %e, stripe_sub_id, "stripe cancel_subscription failed");
                e.to_string()
            })?;
        Ok(())
    }

    /// Swap the price on the first item of a Stripe subscription (tier upgrade/downgrade).
    /// Uses Stripe's default proration so the customer is billed/credited correctly.
    pub async fn change_subscription_price(
        &self,
        stripe_sub_id: &str,
        new_price_id: &str,
    ) -> Result<(), String> {
        use stripe::{Subscription, SubscriptionId, UpdateSubscription, UpdateSubscriptionItems};
        let id: SubscriptionId = stripe_sub_id
            .parse()
            .map_err(|e: stripe::ParseIdError| e.to_string())?;

        let sub = Subscription::retrieve(&self.client, &id, &[])
            .await
            .map_err(|e| {
                error!(error = %e, stripe_sub_id, "stripe retrieve subscription failed");
                e.to_string()
            })?;

        let item_id = sub
            .items
            .data
            .first()
            .map(|i| i.id.to_string())
            .ok_or_else(|| "subscription has no items".to_string())?;

        let mut params = UpdateSubscription::new();
        params.items = Some(vec![UpdateSubscriptionItems {
            id: Some(item_id),
            price: Some(new_price_id.to_string()),
            ..Default::default()
        }]);

        Subscription::update(&self.client, &id, params)
            .await
            .map_err(|e| {
                error!(error = %e, stripe_sub_id, "stripe change_subscription_price failed");
                e.to_string()
            })?;
        Ok(())
    }

    // ── Receipt ───────────────────────────────────────────────────────────────

    /// Retrieve the Stripe-hosted receipt URL for a completed payment intent.
    /// Returns `None` if the charge has no receipt (e.g. not yet captured).
    pub async fn get_receipt_url(&self, payment_intent_id: &str) -> Result<Option<String>, String> {
        use stripe::{Expandable, PaymentIntent, PaymentIntentId};
        let id: PaymentIntentId = payment_intent_id
            .parse()
            .map_err(|e: stripe::ParseIdError| e.to_string())?;
        let pi = PaymentIntent::retrieve(&self.client, &id, &["latest_charge"])
            .await
            .map_err(|e| {
                error!(error = %e, payment_intent_id, "stripe get_receipt_url failed");
                e.to_string()
            })?;
        Ok(pi.latest_charge.and_then(|c| match c {
            Expandable::Object(charge) => charge.receipt_url,
            _ => None,
        }))
    }

    // ── Webhook ───────────────────────────────────────────────────────────────

    pub fn construct_event(&self, payload: &str, signature: &str) -> Result<Event, String> {
        Webhook::construct_event(payload, signature, &self.webhook_secret).map_err(|e| {
            error!(error = %e, "stripe webhook construct_event failed");
            e.to_string()
        })
    }

    fn resolve_redirect_target(&self, target: &str) -> Result<String, String> {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            return Err("redirect URL/path is empty".to_string());
        }
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return Ok(trimmed.to_string());
        }
        Self::ensure_valid_portal_url(&self.portal_url)?;
        Ok(join_url(&self.portal_url, trimmed))
    }
    /// Create a one-time price for an **existing** Stripe product.
    /// Used when rotating a price (old archived, new one takes its place)
    /// so the same Stripe product is reused rather than spawning a new one.
    pub async fn create_price_for_existing_product(
        &self,
        stripe_product_id: &str,
        unit_amount: i64,
        currency: &str,
    ) -> Result<Price, String> {
        use stripe::ProductId as StripeProductId;
        use stripe::{CreatePrice, IdOrCreate};
        let product_id: StripeProductId = stripe_product_id
            .parse()
            .map_err(|e: stripe::ParseIdError| e.to_string())?;
        let currency: Currency = currency.parse().unwrap_or(Currency::USD);
        let mut params = CreatePrice::new(currency);
        params.unit_amount = Some(unit_amount);
        params.product = Some(IdOrCreate::Id(&product_id));
        Price::create(&self.client, params).await.map_err(|e| {
            error!(error = %e, "stripe create_price_for_existing_product failed");
            e.to_string()
        })
    }

    /// Permanently delete a Stripe Product.
    /// Archive all associated prices before calling this.
    pub async fn delete_product(&self, stripe_product_id: &str) -> Result<(), String> {
        use stripe::ProductId;
        let id: ProductId = stripe_product_id
            .parse()
            .map_err(|e: stripe::ParseIdError| e.to_string())?;
        stripe::Product::delete(&self.client, &id)
            .await
            .map_err(|e| {
                error!(error = %e, stripe_product_id, "stripe delete_product failed");
                e.to_string()
            })?;
        Ok(())
    }

    /// Set a Stripe product to active (purchasable in checkout).
    pub async fn activate_product(&self, stripe_product_id: &str) -> Result<(), String> {
        use stripe::{ProductId, UpdateProduct};
        let id: ProductId = stripe_product_id
            .parse()
            .map_err(|e: stripe::ParseIdError| e.to_string())?;
        let mut params = UpdateProduct::new();
        params.active = Some(true);
        stripe::Product::update(&self.client, &id, params)
            .await
            .map_err(|e| {
                error!(error = %e, stripe_product_id, "stripe activate_product failed");
                e.to_string()
            })?;
        Ok(())
    }

    /// Set a Stripe product to inactive (hidden from checkout).
    pub async fn deactivate_product(&self, stripe_product_id: &str) -> Result<(), String> {
        use stripe::{ProductId, UpdateProduct};
        let id: ProductId = stripe_product_id
            .parse()
            .map_err(|e: stripe::ParseIdError| e.to_string())?;
        let mut params = UpdateProduct::new();
        params.active = Some(false);
        stripe::Product::update(&self.client, &id, params)
            .await
            .map_err(|e| {
                error!(error = %e, stripe_product_id, "stripe deactivate_product failed");
                e.to_string()
            })?;
        Ok(())
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

fn join_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    if path.starts_with('/') {
        format!("{base}{path}")
    } else {
        format!("{base}/{path}")
    }
}

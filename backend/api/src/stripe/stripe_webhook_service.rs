use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use stripe::{
    CheckoutSessionMode, CheckoutSessionPaymentStatus, Event, EventObject, EventType, Expandable,
};
use tracing::{error, info, warn};

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_product::ProductId;
use crate::domain::Tier;
use crate::invoices::create_invoice_params::CreateInvoiceParams;
use crate::invoices::invoices_provider::InvoicesProvider;
use crate::purchased_products::data_provider::purchased_products_data_provider::PurchasedProductsDataProvider;
use crate::rate_limits_config::RateLimitRefillInput;
use crate::stripe::rate_limit_service::rate_limit_service::RateLimitService;
use crate::stripe::stripe_webhook_handler::StripeWebhookHandler;
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;
use crate::subscriptions::subscriptions_provider::SubscriptionsProvider;
use crate::subscriptions::update_subscription_params::UpdateSubscriptionParams;

pub struct StripeWebhookServiceImpl {
    purchased_products_provider: Arc<dyn PurchasedProductsDataProvider>,
    subscriptions_provider: Arc<dyn SubscriptionsProvider>,
    invoices_provider: Arc<dyn InvoicesProvider>,
    rate_limit_service: Arc<dyn RateLimitService>,
}

impl StripeWebhookServiceImpl {
    pub fn new(
        purchased_products_provider: Arc<dyn PurchasedProductsDataProvider>,
        subscriptions_provider: Arc<dyn SubscriptionsProvider>,
        invoices_provider: Arc<dyn InvoicesProvider>,
        rate_limit_service: Arc<dyn RateLimitService>,
    ) -> Self {
        Self {
            purchased_products_provider,
            subscriptions_provider,
            invoices_provider,
            rate_limit_service,
        }
    }

    // ── Shared helpers ─────────────────────────────────────────────────────────

    /// Resolves org_id by looking up a stripe subscription ID in the DB.
    /// Returns `Ok(None)` when no subscription row matches (caller should warn + skip).
    async fn resolve_org_id_by_stripe_sub_id(
        &self,
        stripe_sub_id: &str,
    ) -> Result<Option<OrganizationId>, String> {
        match self
            .subscriptions_provider
            .find_by_stripe_subscription_id(stripe_sub_id)
            .await
        {
            Ok(Some(sub)) => {
                let org_id = OrganizationId::try_from(i64::from(sub.org_id))
                    .map_err(|_| format!("invalid org_id for subscription {stripe_sub_id}"))?;
                Ok(Some(org_id))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("failed to find subscription {stripe_sub_id}: {e}")),
        }
    }

    /// Updates only the `status` column of the subscription row for `org_id`.
    async fn set_subscription_status(
        &self,
        org_id: OrganizationId,
        status: &str,
    ) -> Result<(), String> {
        self.subscriptions_provider
            .update_subscription(
                org_id,
                UpdateSubscriptionParams {
                    tier: None,
                    stripe_subscription_id: None,
                    status: Some(status.to_string()),
                    current_period_end: None,
                },
            )
            .await
            .map_err(|e| format!("failed to set subscription status to '{status}': {e}"))?;
        Ok(())
    }

    // ── Checkout handlers ──────────────────────────────────────────────────────

    async fn handle_checkout_completed(
        &self,
        session: stripe::CheckoutSession,
    ) -> Result<(), String> {
        match session.mode {
            CheckoutSessionMode::Payment => self.handle_checkout_payment(session).await,
            CheckoutSessionMode::Subscription => self.handle_checkout_subscription(session).await,
            _ => {
                warn!("checkout.session.completed with unexpected mode");
                Ok(())
            }
        }
    }

    async fn handle_checkout_payment(
        &self,
        session: stripe::CheckoutSession,
    ) -> Result<(), String> {
        let metadata = &session.metadata;

        let org_id = parse_meta_i64(metadata, "org_id");
        let product_id = parse_meta_i64(metadata, "product_id");
        let tokens_granted = parse_meta_i64(metadata, "tokens_granted");

        let (Some(org_id), Some(product_id), Some(tokens_granted)) =
            (org_id, product_id, tokens_granted)
        else {
            return Err(
                "checkout.session.completed(payment): missing metadata fields".to_string(),
            );
        };

        let payment_intent_id = match &session.payment_intent {
            Some(Expandable::Id(id)) => Some(id.to_string()),
            Some(Expandable::Object(pi)) => Some(pi.id.to_string()),
            None => None,
        };

        let org_id = OrganizationId::try_from(org_id)
            .map_err(|_| "invalid org_id in metadata".to_string())?;
        let product_id = ProductId::try_from(product_id)
            .map_err(|_| "invalid product_id in metadata".to_string())?;

        self.purchased_products_provider
            .create_purchased_product(org_id, product_id, tokens_granted, payment_intent_id)
            .await
            .map_err(|e| format!("failed to record purchased product: {e}"))?;

        let amount = u64::try_from(tokens_granted)
            .map_err(|_| "invalid tokens_granted in metadata".to_string())?;
        if amount == 0 {
            return Err("tokens_granted must be > 0".to_string());
        }

        self.rate_limit_service
            .apply_refill(org_id, RateLimitRefillInput::BucketAdd { amount })
            .await
            .map_err(|e| format!("failed to apply token refill after purchase: {e}"))?;
        Ok(())
    }

    async fn handle_checkout_subscription(
        &self,
        session: stripe::CheckoutSession,
    ) -> Result<(), String> {
        let metadata = &session.metadata;
        let org_id = parse_meta_i64(metadata, "org_id");
        let tier: Option<Tier> = metadata
            .as_ref()
            .and_then(|m| m.get("tier"))
            .and_then(|s| s.parse::<Tier>().ok());

        let (Some(org_id), Some(tier)) = (org_id, tier) else {
            return Err("checkout.session.completed(subscription): missing or invalid metadata fields".to_string());
        };

        let stripe_sub_id = match &session.subscription {
            Some(Expandable::Id(id)) => id.to_string(),
            Some(Expandable::Object(s)) => s.id.to_string(),
            None => {
                return Err(
                    "checkout.session.completed(subscription): no subscription id".to_string(),
                );
            }
        };

        let org_id = OrganizationId::try_from(org_id)
            .map_err(|_| "invalid org_id in metadata".to_string())?;

        // For 3DS flows, payment_status may be "unpaid" at this point.
        // We still create/update the row so customer.subscription.updated can transition it.
        let status = if session.payment_status == CheckoutSessionPaymentStatus::Paid {
            "active"
        } else {
            warn!(%stripe_sub_id, "checkout.session.completed(subscription): payment not yet paid, creating as incomplete");
            "incomplete"
        };

        // Upsert: try update first, create if not found
        let update_result = self
            .subscriptions_provider
            .update_subscription(
                org_id,
                UpdateSubscriptionParams {
                    tier: Some(tier.clone()),
                    stripe_subscription_id: Some(stripe_sub_id.clone()),
                    status: Some(status.to_string()),
                    current_period_end: None,
                },
            )
            .await;

        match update_result {
            Ok(Some(_)) => info!(%stripe_sub_id, status, "subscription updated"),
            Ok(None) => {
                self.subscriptions_provider
                    .create_subscription(CreateSubscriptionParams {
                        org_id,
                        tier,
                        stripe_subscription_id: Some(stripe_sub_id.clone()),
                        status: status.to_string(),
                        current_period_end: None,
                    })
                    .await
                    .map_err(|e| format!("failed to create subscription: {e}"))?;
                info!(%stripe_sub_id, status, "subscription created");
            }
            Err(e) => return Err(format!("failed to upsert subscription on checkout: {e}")),
        }

        Ok(())
    }

    // ── Subscription handlers ──────────────────────────────────────────────────

    async fn handle_subscription_updated(
        &self,
        sub: stripe::Subscription,
    ) -> Result<(), String> {
        let org_id =
            org_id_from_sub_metadata(&sub.metadata, "customer.subscription.updated")?;
        let tier: Option<Tier> = sub.metadata.get("tier").and_then(|s| s.parse::<Tier>().ok());
        let status = sub.status.to_string();
        let period_end =
            chrono::DateTime::from_timestamp(sub.current_period_end, 0).map(|dt| dt.to_rfc3339());

        self.subscriptions_provider
            .update_subscription(
                org_id,
                UpdateSubscriptionParams {
                    tier,
                    stripe_subscription_id: Some(sub.id.to_string()),
                    status: Some(status),
                    current_period_end: period_end,
                },
            )
            .await
            .map_err(|e| format!("failed to sync subscription from webhook: {e}"))?;

        info!(stripe_sub_id = %sub.id, "subscription synced from webhook");
        Ok(())
    }

    async fn handle_subscription_deleted(
        &self,
        sub: stripe::Subscription,
    ) -> Result<(), String> {
        let org_id =
            org_id_from_sub_metadata(&sub.metadata, "customer.subscription.deleted")?;
        self.set_subscription_status(org_id, "cancelled").await?;
        self.rate_limit_service
            .apply_refill(org_id, RateLimitRefillInput::BucketReset)
            .await
            .map_err(|e| format!("customer.subscription.deleted: failed to reset token bucket: {e}"))?;
        info!(stripe_sub_id = %sub.id, "subscription marked canceled");
        Ok(())
    }

    // ── Invoice handlers ───────────────────────────────────────────────────────

    async fn handle_invoice_payment_failed(
        &self,
        invoice: stripe::Invoice,
    ) -> Result<(), String> {
        let stripe_sub_id = match sub_id_from_invoice_subscription(&invoice) {
            Some(id) => id,
            None => {
                warn!("invoice.payment_failed: no subscription on invoice");
                return Ok(());
            }
        };

        let org_id = match self.resolve_org_id_by_stripe_sub_id(&stripe_sub_id).await? {
            Some(id) => id,
            None => {
                warn!(%stripe_sub_id, "invoice.payment_failed: no matching subscription found");
                return Ok(());
            }
        };

        self.set_subscription_status(org_id, "past_due")
            .await
            .map_err(|e| format!("invoice.payment_failed: {e}"))?;
        info!(%stripe_sub_id, "subscription marked past_due");
        Ok(())
    }

    async fn handle_invoice_payment_action_required(
        &self,
        invoice: stripe::Invoice,
    ) -> Result<(), String> {
        let stripe_invoice_id = invoice.id.to_string();

        let stripe_sub_id = match sub_id_from_invoice_subscription(&invoice) {
            Some(id) => id,
            None => {
                warn!(%stripe_invoice_id, "invoice.payment_action_required: no subscription on invoice");
                return Ok(());
            }
        };

        let org_id = match self.resolve_org_id_by_stripe_sub_id(&stripe_sub_id).await? {
            Some(id) => id,
            None => {
                warn!(%stripe_sub_id, "invoice.payment_action_required: no matching subscription found");
                return Ok(());
            }
        };

        self.set_subscription_status(org_id, "action_required")
            .await
            .map_err(|e| format!("invoice.payment_action_required: {e}"))?;
        info!(%stripe_sub_id, "subscription marked action_required");
        Ok(())
    }

    async fn handle_invoice_paid(&self, invoice: stripe::Invoice) -> Result<(), String> {
        let stripe_invoice_id = invoice.id.to_string();

        let stripe_sub_id = match sub_id_from_invoice_subscription(&invoice) {
            Some(id) => id,
            None => {
                info!(%stripe_invoice_id, "invoice.paid: no subscription attached, skipping");
                return Ok(());
            }
        };

        let amount = invoice.amount_paid.unwrap_or(0);
        let currency = invoice
            .currency
            .map(|c| c.to_string())
            .unwrap_or_else(|| "usd".to_string());
        let status = invoice
            .status
            .map(|s| s.to_string())
            .unwrap_or_else(|| "paid".to_string());
        let hosted_url = invoice.hosted_invoice_url;

        let org_id = match self.resolve_org_id_by_stripe_sub_id(&stripe_sub_id).await? {
            Some(id) => id,
            None => {
                warn!(%stripe_sub_id, "invoice.paid: no matching subscription found");
                return Ok(());
            }
        };

        let inv = self
            .invoices_provider
            .create_invoice(CreateInvoiceParams {
                org_id,
                stripe_invoice_id: stripe_invoice_id.clone(),
                amount,
                currency,
                status,
                hosted_url,
            })
            .await
            .map_err(|e| format!("failed to record invoice {stripe_invoice_id}: {e}"))?;

        info!(invoice_id = ?inv.id, %stripe_sub_id, "invoice recorded");
        Ok(())
    }

}

#[async_trait]
impl StripeWebhookHandler for StripeWebhookServiceImpl {
    async fn handle_event(&self, event: Event) {
        match event.type_ {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    if let Err(e) = self.handle_checkout_completed(session).await {
                        error!(error = %e, "checkout.session.completed handler failed");
                    }
                }
            }
            EventType::CustomerSubscriptionUpdated => {
                if let EventObject::Subscription(sub) = event.data.object {
                    if let Err(e) = self.handle_subscription_updated(sub).await {
                        error!(error = %e, "customer.subscription.updated handler failed");
                    }
                }
            }
            EventType::CustomerSubscriptionDeleted => {
                if let EventObject::Subscription(sub) = event.data.object {
                    if let Err(e) = self.handle_subscription_deleted(sub).await {
                        error!(error = %e, "customer.subscription.deleted handler failed");
                    }
                }
            }
            EventType::InvoicePaymentFailed => {
                if let EventObject::Invoice(invoice) = event.data.object {
                    if let Err(e) = self.handle_invoice_payment_failed(invoice).await {
                        error!(error = %e, "invoice.payment_failed handler failed");
                    }
                }
            }
            EventType::InvoicePaymentActionRequired => {
                if let EventObject::Invoice(invoice) = event.data.object {
                    if let Err(e) = self.handle_invoice_payment_action_required(invoice).await {
                        error!(error = %e, "invoice.payment_action_required handler failed");
                    }
                }
            }
            EventType::InvoicePaid => {
                if let EventObject::Invoice(invoice) = event.data.object {
                    if let Err(e) = self.handle_invoice_paid(invoice).await {
                        error!(error = %e, "invoice.paid handler failed");
                    }
                }
            }
            _ => {
                info!(event_type = ?event.type_, "unhandled stripe event — ignoring");
            }
        }
    }
}

// ── Free helpers ───────────────────────────────────────────────────────────────

/// Extracts the subscription ID string from an invoice's subscription expandable.
fn sub_id_from_invoice_subscription(invoice: &stripe::Invoice) -> Option<String> {
    match &invoice.subscription {
        Some(Expandable::Id(id)) => Some(id.to_string()),
        Some(Expandable::Object(s)) => Some(s.id.to_string()),
        None => None,
    }
}

/// Parses and validates the `org_id` from a subscription's metadata map.
fn org_id_from_sub_metadata(
    metadata: &HashMap<String, String>,
    context: &str,
) -> Result<OrganizationId, String> {
    let raw: i64 = metadata
        .get("org_id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| format!("{context}: missing org_id in subscription metadata"))?;
    OrganizationId::try_from(raw)
        .map_err(|_| format!("{context}: invalid org_id in subscription metadata"))
}

fn parse_meta_i64(
    metadata: &Option<HashMap<String, String>>,
    key: &str,
) -> Option<i64> {
    let m = metadata.as_ref()?;
    let s = m.get(key)?;
    s.parse::<i64>().ok()
}

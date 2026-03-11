use actix_web::{post, web, HttpRequest, HttpResponse};
use tracing::{info, warn};

use crate::stripe::stripe_webhook_app_data::StripeWebhookAppData;

// ── AppData ───────────────────────────────────────────────────────────────────

// ── Routing ───────────────────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_stripe_webhook);
}

// ── Handler ───────────────────────────────────────────────────────────────────

/// POST /webhooks/stripe
/// No JWT auth — Stripe signature validates authenticity.
#[post("")]
pub async fn handle_stripe_webhook(
    req: HttpRequest,
    body: web::Bytes,
    app: web::Data<StripeWebhookAppData>,
) -> HttpResponse {
    let signature = match req.headers().get("Stripe-Signature") {
        Some(v) => match v.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return HttpResponse::BadRequest().body("invalid Stripe-Signature header"),
        },
        None => return HttpResponse::BadRequest().body("missing Stripe-Signature header"),
    };

    let payload = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest().body("invalid utf-8 body"),
    };

    let event = match app.stripe.construct_event(payload, &signature) {
        Ok(e) => e,
        Err(e) => {
            warn!(error = %e, "stripe webhook signature validation failed");
            return HttpResponse::BadRequest().body("invalid signature");
        }
    };

    info!(event_type = ?event.type_, event_id = %event.id, "stripe webhook received");

    app.webhook_service.handle_event(event).await;

    HttpResponse::Ok().finish()
}

use actix_web::{get, web, HttpResponse};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use utoipa::path;

static STARTED_AT: Lazy<Instant> = Lazy::new(Instant::now);
static HTTP_REQUESTS_TOTAL: AtomicU64 = AtomicU64::new(0);

pub fn inc_http_requests_total() {
    HTTP_REQUESTS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

#[path(
    get,
    path = "/metrics",
    tag = "metrics",
    responses((status = 200, description = "Prometheus metrics", body = String))
)]
#[get("")]
pub async fn metrics() -> HttpResponse {
    let uptime_seconds = STARTED_AT.elapsed().as_secs();
    let http_requests_total = HTTP_REQUESTS_TOTAL.load(Ordering::Relaxed);

    let body = format!(
        concat!(
            "# HELP backend_uptime_seconds Backend uptime in seconds.\n",
            "# TYPE backend_uptime_seconds counter\n",
            "backend_uptime_seconds {uptime_seconds}\n",
            "# HELP http_requests_total Total HTTP requests served by backend.\n",
            "# TYPE http_requests_total counter\n",
            "http_requests_total {http_requests_total}\n"
        )
    );

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(body)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(metrics);
}

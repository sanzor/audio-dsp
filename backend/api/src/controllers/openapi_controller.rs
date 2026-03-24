use actix_web::{get, web, HttpResponse};
use utoipa::OpenApi;

use crate::openapi::ApiDoc;

#[get("/openapi.json")]
pub async fn openapi_json() -> HttpResponse {
    HttpResponse::Ok().json(ApiDoc::openapi())
}

#[get("/docs")]
pub async fn docs() -> HttpResponse {
    let html = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Audio DSP API Docs</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist/swagger-ui.css" />
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
    <script>
      window.ui = SwaggerUIBundle({
        url: '/openapi.json',
        dom_id: '#swagger-ui',
        persistAuthorization: true,
      });
    </script>
  </body>
</html>
"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(openapi_json).service(docs);
}

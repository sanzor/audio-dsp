use actix_web::{web, App, HttpServer, Route};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let v =
        HttpServer::new(|| App::new().service(web::scope("/player").route("play", Route::new())))
            .bind(("127.0.0.1", 8080));
    Ok(())
}

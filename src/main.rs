use actix_web::{web, App, HttpServer};

mod controllers;
mod models;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/process_payment", web::post().to(controllers::payment_controller::process_payment))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

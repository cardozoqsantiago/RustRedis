#![allow(non_snake_case)]
use actix_web::{App, HttpServer};
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(controllers::reader_controller::reader_controller)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


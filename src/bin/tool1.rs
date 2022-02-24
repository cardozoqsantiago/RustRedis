#![allow(non_snake_case)]
#[macro_use]
extern crate lazy_static;

use actix_web::{App, HttpServer};
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(controllers::record_controller::reader_controller)
            .service(controllers::record_controller::record_controller)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}



use actix_web::web::Path;
use actix_web::{web, Responder, get, post,HttpResponse};
#[path = "../services/record_service.rs"] 
mod services;


#[get("/read/{number_request}")]
async fn reader_controller(path: Path<(String,)>) -> impl Responder {
    let name_path = path.0 .0;
    let full_path = format!("{}{}{}", "./", name_path, ".csv");
    return web::Json(services::read_csv_file(full_path));
}

#[post("/record/")]
async fn record_controller() -> HttpResponse {
    services::add_record("", "date: &str", 12, 12, 12);
    HttpResponse::Ok()
    .content_type("application/json")
    .await
    .unwrap()
}

#[post("/conecction/")]
async fn connect() -> HttpResponse {
    services::connect();
    HttpResponse::Ok()
    .content_type("application/json")
    .await
    .unwrap()
}
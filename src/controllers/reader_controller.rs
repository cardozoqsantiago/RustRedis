use actix_web::web::Path;
use actix_web::{web, Responder, get};
#[path = "../services/reader_service.rs"] 
mod services;


#[get("/read/{number_request}")]
async fn reader_controller(path: Path<(String,)>) -> impl Responder {
    let name_path = path.0 .0;
    let full_path = format!("{}{}{}", "./", name_path, ".csv");
    return web::Json(services::read_csv_file(full_path));
}
use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use application::interfaces::i_logger::ILogger;

#[post("/something")]
pub async fn something(log: web::Data<Arc<dyn ILogger>>) -> impl Responder {
    log.debug(format!("Logg Something"));
    HttpResponse::Ok().body("response")
}

use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use application::interfaces::i_logger::ILogger;
use business::usecases::i_something::ISomethingUseCase;

use crate::models::something::SomethingModel;

#[post("/something")]
pub async fn something(
    log: web::Data<Arc<dyn ILogger>>,
    model: web::Json<SomethingModel>,
    use_case: web::Data<Arc<dyn ISomethingUseCase>>,
) -> impl Responder {
    log.debug(format!("Logg Something"));
    use_case.perform(model.as_dto());
    HttpResponse::Ok().body("response")
}

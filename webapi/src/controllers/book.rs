use crate::models::book::BookModel;
use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use application::interfaces::i_logger::ILogger;
use business::usecases::i_book::IBookUseCase;

#[post("/api/v1/book")]
pub async fn create_book(
    logger: web::Data<Arc<dyn ILogger>>,
    model: web::Json<BookModel>,
    use_case: web::Data<Arc<dyn IBookUseCase>>,
) -> impl Responder {
    logger.info("controller::create_book", "Logg");
    use_case.perform(model.as_dto());
    HttpResponse::Ok().body("response")
}

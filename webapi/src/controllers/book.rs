use crate::models::{
    book::{BookModel, ResponseCreateBookModel},
    http_error::HttpError,
};
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
    logger.info("[Controller::create_book]", "...");

    let result = use_case.perform(model.to_create_book_dto());

    if let Err(_) = result {
        return HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            details: "Some error occur".to_string(),
            message: "Try latter".to_string(),
        });
    }

    return HttpResponse::Ok().json(ResponseCreateBookModel::from_book(result.unwrap()));
}

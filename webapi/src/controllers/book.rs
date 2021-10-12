use crate::models::{
    book::{BookModel, ResponseCreateBookModel},
    http_error::HttpError,
};
use std::sync::Arc;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
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

#[get("/api/v1/book")]
pub async fn get_book_by_id(
    _logger: web::Data<Arc<dyn ILogger>>,
    _model: web::Json<BookModel>,
    _use_case: web::Data<Arc<dyn IBookUseCase>>,
) -> impl Responder {
    HttpResponse::Ok().body("GET /api/v1/book")
}

#[put("/api/v1/book")]
pub async fn update_book(
    _logger: web::Data<Arc<dyn ILogger>>,
    _model: web::Json<BookModel>,
    _use_case: web::Data<Arc<dyn IBookUseCase>>,
) -> impl Responder {
    HttpResponse::Ok().body("PUT /api/v1/book")
}

#[delete("/api/v1/book")]
pub async fn delete_book(
    _logger: web::Data<Arc<dyn ILogger>>,
    _model: web::Json<BookModel>,
    _use_case: web::Data<Arc<dyn IBookUseCase>>,
) -> impl Responder {
    HttpResponse::Ok().body("DELETE /api/v1/book")
}

use crate::models::{
    book::{CreateBookModel, ResponseCreateBookModel, UpdateBookModel},
    http_error::HttpError,
};
use std::sync::Arc;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use application::interfaces::i_logger::ILogger;
use business::usecases::i_book::{
    ICreateBookUseCase, IDeleteBookUseCase, IGetBookUseCase, IUpdateBookUseCase,
};

#[post("/api/v1/books")]
pub async fn create_book(
    logger: web::Data<Arc<dyn ILogger>>,
    model: web::Json<CreateBookModel>,
    use_case: web::Data<Arc<dyn ICreateBookUseCase>>,
) -> impl Responder {
    logger.info("[Controller::create_book]", "...");
    match use_case.perform(model.to_create_book_dto()) {
        Ok(book) => HttpResponse::Ok().json(ResponseCreateBookModel::from_book(book)),
        _ => HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            details: "Some error occur".to_string(),
            message: "Bad Request".to_string(),
        }),
    }
}

#[get("/api/v1/books/{id}")]
pub async fn get_book_by_id(
    web::Path(id): web::Path<i32>,
    use_case: web::Data<Arc<dyn IGetBookUseCase>>,
) -> impl Responder {
    println!("ok");
    match use_case.perform(id) {
        Ok(book) => match book {
            Some(book) => HttpResponse::BadRequest().json(ResponseCreateBookModel::from_book(book)),
            None => HttpResponse::BadRequest().json(HttpError {
                status_code: 404,
                details: "The id not belongs this application".to_string(),
                message: "Not Found".to_string(),
            }),
        },
        _ => HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            details: "Some error occur".to_string(),
            message: "Bad Request".to_string(),
        }),
    }
}

#[put("/api/v1/books/{id}")]
pub async fn update_book(
    web::Path(id): web::Path<i32>,
    model: web::Json<UpdateBookModel>,
    use_case: web::Data<Arc<dyn IUpdateBookUseCase>>,
) -> impl Responder {
    match use_case.perform(id, model.to_update_book_dto()) {
        Ok(updated) if updated => HttpResponse::Ok().json({}),
        Ok(updated) if !updated => HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            details: "The id not belongs this application".to_string(),
            message: "Bad Request".to_string(),
        }),
        _ => HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            details: "Some error occur".to_string(),
            message: "Bad Request".to_string(),
        }),
    }
}

#[delete("/api/v1/books/{id}")]
pub async fn delete_book(
    web::Path(id): web::Path<i32>,
    use_case: web::Data<Arc<dyn IDeleteBookUseCase>>,
) -> impl Responder {
    match use_case.perform(id) {
        Ok(deleted) if deleted => HttpResponse::Ok().json({}),
        Ok(deleted) if !deleted => HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            details: "The id not belongs this application".to_string(),
            message: "Bad Request".to_string(),
        }),
        _ => HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            details: "Some error occur".to_string(),
            message: "Bad Request".to_string(),
        }),
    }
}

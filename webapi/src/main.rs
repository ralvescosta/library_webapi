mod controllers;
mod middleware;
mod models;

use application::{
    interfaces::i_logger::ILogger,
    usecases::book::{CreateBookUseCase, DeleteBookUseCase, GetBookUseCase, UpdateBookUseCase},
};
use business::usecases::i_book::{
    ICreateBookUseCase, IDeleteBookUseCase, IGetBookUseCase, IUpdateBookUseCase,
};
use infrastructure::{
    database, environments, logger::logger::Logger, repositories::book_repository::BookRepository,
};

use actix_cors::Cors;
use actix_web::{http, middleware as actix_middleware, web, App, HttpServer};

use std::{io::Result, sync::Arc};

#[actix_web::main]
async fn main() -> Result<()> {
    Logger::init();

    HttpServer::new(|| {
        environments::env::register_env().unwrap();
        let logger = Arc::new(Logger::new());
        let coon_pool = Arc::new(database::connection::create_connection_pool());
        let book_repository = Arc::new(BookRepository::new(logger.clone(), coon_pool.clone()));
        let create_book = Arc::new(CreateBookUseCase::new(
            logger.clone(),
            book_repository.clone(),
        ));
        let get_book = Arc::new(GetBookUseCase::new(book_repository.clone()));
        let update_book = Arc::new(UpdateBookUseCase::new(book_repository.clone()));
        let delete_book = Arc::new(DeleteBookUseCase::new(book_repository.clone()));

        App::new()
            .wrap(actix_middleware::Logger::default())
            .wrap(actix_middleware::Compress::default())
            .wrap(config_cors())
            .wrap(config_headers())
            .data(middleware::deserializer_error::handler())
            // Injections
            .app_data(web::Data::<Arc<dyn ILogger>>::new(logger))
            .app_data(web::Data::<Arc<dyn ICreateBookUseCase>>::new(create_book))
            .app_data(web::Data::<Arc<dyn IGetBookUseCase>>::new(get_book))
            .app_data(web::Data::<Arc<dyn IUpdateBookUseCase>>::new(update_book))
            .app_data(web::Data::<Arc<dyn IDeleteBookUseCase>>::new(delete_book))
            // routes
            .service(controllers::book::create_book)
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}

fn config_cors() -> Cors {
    Cors::default()
        .allowed_origin("All")
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::ORIGIN,
            http::header::LOCATION,
            http::header::HOST,
            http::header::USER_AGENT,
            http::header::CONTENT_LENGTH,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
}

fn config_headers() -> actix_middleware::DefaultHeaders {
    actix_middleware::DefaultHeaders::new()
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Security-Policy", "default-src 'self';base-uri 'self';block-all-mixed-content;font-src 'self' https: data:;frame-ancestors 'self';img-src 'self' data:;object-src 'none';script-src 'self';script-src-attr 'none';style-src 'self' https: 'unsafe-inline';upgrade-insecure-requests")
            .header("X-DNS-Prefetch-Control", "	off")
            .header("Expect-CT", "max-age=0")
            .header("X-Frame-Options", "SAMEORIGIN")
            .header("Strict-Transport-Security", "max-age=15552000; includeSubDomains")
            .header("X-Download-Options", "noopen")
            .header("X-Content-Type-Options", "nosniff")
            .header("X-Permitted-Cross-Domain-Policies", "none")
            .header("Referrer-Policy", "no-referrer")
            .header("X-XSS-Protection","0")
            .header("ETag", "W/\"213-XP2qvFfd8eh4EzgQSHCwnbPqiP4\"")
            .header("Vary", "Accept-Encoding")
}

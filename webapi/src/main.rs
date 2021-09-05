mod controllers;
mod middleware;
mod models;

use application::{interfaces::i_logger::ILogger, usecases::book::BookUseCase};
use business::usecases::i_book::IBookUseCase;
use infrastructure::logger::logger::Logger;
use infrastructure::repositories::book_repository::BookRepository;

use actix_cors::Cors;
use actix_web::{http, middleware as actix_middleware, web, App, HttpServer};
use env_logger;
use std::{io::Result, sync::Arc};

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    HttpServer::new(|| {
        let logger = config_logger();
        let book_repository = Box::new(BookRepository::new(logger.clone()));
        let book_use_case = Arc::new(BookUseCase::new(logger.clone(), book_repository));

        App::new()
            .wrap(actix_middleware::Logger::default())
            .wrap(actix_middleware::Compress::default())
            .wrap(config_cors())
            .wrap(config_headers())
            .data(logger)
            .data(middleware::deserializer_error::handler())
            // Injections
            .app_data(web::Data::<Arc<dyn IBookUseCase>>::new(book_use_case))
    })
    .bind("127.0.0.1:3000")?
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

fn config_logger() -> Arc<dyn ILogger> {
    Arc::new(Logger::new())
}

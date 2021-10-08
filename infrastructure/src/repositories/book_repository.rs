use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use std::sync::Arc;

use application::interfaces::i_book_repository::IBookRepository;
use application::interfaces::i_logger::ILogger;
use business::dtos::create_book_dto::CreateBookDto;
use business::entities::book::Book;

use crate::database::models::books::{BookModel, CreateBookModel};

pub struct BookRepository {
    logger: Arc<dyn ILogger>,
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
}

impl IBookRepository for BookRepository {
    fn create(&self, dto: CreateBookDto) -> Book {
        use crate::schema::books;

        self.logger.info("BookRepository", "Logging");
        let connection = self.pool.get().unwrap();

        let result: BookModel = diesel::insert_into(books::table)
            .values(CreateBookModel::from_create_book_dto(dto))
            .get_result(&connection)
            .expect("Error saving");

        result.to_book()
    }
}

impl BookRepository {
    pub fn new(
        logger: Arc<dyn ILogger>,
        pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
    ) -> impl IBookRepository {
        BookRepository { logger, pool }
    }
}

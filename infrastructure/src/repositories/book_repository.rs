use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use std::sync::Arc;

use application::interfaces::i_book_repository::IBookRepository;
use application::interfaces::i_logger::ILogger;
use business::dtos::create_book_dto::CreateBookDto;
use business::entities::book::Book;

pub struct BookRepository {
    logger: Arc<dyn ILogger>,
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
}

impl IBookRepository for BookRepository {
    fn create(&self, dto: CreateBookDto) -> Book {
        use crate::schema::books;

        self.logger.info("BookRepository", "Logging");
        let connection = self.pool.get().unwrap();

        let book = Book::from_dto(dto);

        diesel::insert_into(books::table)
            .values(&dto)
            .get_result(&connection)
            .expect("Error saving");
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

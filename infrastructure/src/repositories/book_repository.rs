use application::interfaces::i_book_repository::IBookRepository;
use application::interfaces::i_logger::ILogger;
use business::dtos::create_book_dto::CreateBookDto;
use business::entities::book::Book;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::sync::Arc;

pub struct BookRepository {
    logger: Arc<dyn ILogger>,
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
}

impl IBookRepository for BookRepository {
    fn create(&self, _dto: CreateBookDto) -> Book {
        self.logger.info("BookRepository", "Logging");
        let _connection = self.pool.get().unwrap();

        Book {}
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

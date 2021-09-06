use application::interfaces::i_book_repository::IBookRepository;
use application::interfaces::i_logger::ILogger;
use business::dtos::create_book_dto::CreateBookDto;
use business::entities::book::Book;
use std::sync::Arc;

pub struct BookRepository {
    logger: Arc<dyn ILogger>,
}

impl IBookRepository for BookRepository {
    fn create(&self, dto: CreateBookDto) -> Book {
        self.logger.info("BookRepository", "Logg");
        Book {}
    }
}

impl BookRepository {
    pub fn new(logger: Arc<dyn ILogger>) -> impl IBookRepository {
        BookRepository { logger }
    }
}

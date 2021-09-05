use std::sync::Arc;
use application::interfaces::i_logger::ILogger;
use application::interfaces::i_book_repository::IBookRepository;
use business::entities::book::Book;

pub struct BookRepository {
    logger: Arc<dyn ILogger>
}

impl IBookRepository for BookRepository {
    fn create(&self) -> Book {
        self.logger.info("BookRepository".to_string());
        Book{}
    }
}

impl BookRepository {
    pub fn new(logger: Arc<dyn ILogger>) -> BookRepository {
        BookRepository {logger}
    }
}

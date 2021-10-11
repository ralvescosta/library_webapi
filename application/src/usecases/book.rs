use std::sync::Arc;

use crate::interfaces::{i_book_repository::IBookRepository, i_logger::ILogger};
use business::dtos::create_book_dto::CreateBookDto;
use business::entities::book::Book;
use business::usecases::i_book::IBookUseCase;

pub struct BookUseCase {
    logger: Arc<dyn ILogger>,
    repository: Arc<dyn IBookRepository>,
}

impl IBookUseCase for BookUseCase {
    fn perform(&self, dto: CreateBookDto) -> Result<Book, ()> {
        self.logger.info("IBookUseCase", "Logger");
        if let Ok(result) = self.repository.create(dto) {
            return Ok(result);
        }

        Err(())
    }
}
impl BookUseCase {
    pub fn new(
        logger: Arc<dyn ILogger>,
        repository: Arc<dyn IBookRepository>,
    ) -> impl IBookUseCase {
        BookUseCase { logger, repository }
    }
}

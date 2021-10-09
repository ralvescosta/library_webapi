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
    fn perform(&self, dto: CreateBookDto) -> Book {
        self.logger.info("IBookUseCase", "Logger");
        let result = self.repository.create(dto);
        result
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

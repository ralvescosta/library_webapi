use std::sync::Arc;

use crate::interfaces::{i_book_repository::IBookRepository, i_logger::ILogger};
use business::dtos::create_book_dto::CreateBookDto;
use business::usecases::i_book::IBookUseCase;

pub struct BookUseCase {
    logger: Arc<dyn ILogger>,
    repository: Box<dyn IBookRepository>,
}

impl IBookUseCase for BookUseCase {
    fn perform(&self, dto: CreateBookDto) {
        self.logger.info("BookUseCase".to_string());
        self.repository.create();
    }
}
impl BookUseCase {
    pub fn new(logger: Arc<dyn ILogger>, repository: Box<dyn IBookRepository>) -> BookUseCase {
        BookUseCase { logger, repository }
    }
}

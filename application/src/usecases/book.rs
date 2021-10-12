use std::sync::Arc;

use crate::interfaces::{i_book_repository::IBookRepository, i_logger::ILogger};
use business::dtos::create_book_dto::CreateBookDto;
use business::entities::book::Book;
use business::usecases::i_book::ICreateBookUseCase;

pub struct CreateBookUseCase {
    logger: Arc<dyn ILogger>,
    repository: Arc<dyn IBookRepository>,
}

impl ICreateBookUseCase for CreateBookUseCase {
    fn perform(&self, dto: CreateBookDto) -> Result<Book, ()> {
        self.logger.info("IBookUseCase", "Logger");
        if let Ok(result) = self.repository.create(dto) {
            return Ok(result);
        }

        Err(())
    }
}
impl CreateBookUseCase {
    pub fn new(
        logger: Arc<dyn ILogger>,
        repository: Arc<dyn IBookRepository>,
    ) -> impl ICreateBookUseCase {
        CreateBookUseCase { logger, repository }
    }
}

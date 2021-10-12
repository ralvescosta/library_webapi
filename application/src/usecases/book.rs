use std::sync::Arc;

use crate::interfaces::{i_book_repository::IBookRepository, i_logger::ILogger};
use business::dtos::create_book_dto::CreateBookDto;
use business::dtos::update_book_dto::UpdateBookDto;
use business::entities::book::Book;
use business::usecases::i_book::{
    ICreateBookUseCase, IDeleteBookUseCase, IGetBookUseCase, IUpdateBookUseCase,
};

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

pub struct GetBookUseCase {
    repository: Arc<dyn IBookRepository>,
}
impl IGetBookUseCase for GetBookUseCase {
    fn perform(&self, id: i32) -> Result<Option<Book>, ()> {
        match self.repository.get_by_id(id) {
            Ok(book) => Ok(book),
            _ => Err(()),
        }
    }
}
impl GetBookUseCase {
    pub fn new(repository: Arc<dyn IBookRepository>) -> impl IGetBookUseCase {
        GetBookUseCase { repository }
    }
}

pub struct UpdateBookUseCase {
    repository: Arc<dyn IBookRepository>,
}
impl IUpdateBookUseCase for UpdateBookUseCase {
    fn perform(&self, id: i32, dto: UpdateBookDto) -> Result<bool, ()> {
        match self.repository.update(id, dto) {
            Ok(result) => Ok(result),
            _ => Err(()),
        }
    }
}
impl UpdateBookUseCase {
    pub fn new(repository: Arc<dyn IBookRepository>) -> impl IUpdateBookUseCase {
        UpdateBookUseCase { repository }
    }
}

pub struct DeleteBookUseCase {
    repository: Arc<dyn IBookRepository>,
}
impl IDeleteBookUseCase for DeleteBookUseCase {
    fn perform(&self, id: i32) -> Result<bool, ()> {
        match self.repository.delete_by_id(id) {
            Ok(result) => Ok(result),
            _ => Err(()),
        }
    }
}
impl DeleteBookUseCase {
    pub fn new(repository: Arc<dyn IBookRepository>) -> impl IDeleteBookUseCase {
        DeleteBookUseCase { repository }
    }
}

use crate::{
    dtos::{create_book_dto::CreateBookDto, update_book_dto::UpdateBookDto},
    entities::book::Book,
};

pub trait ICreateBookUseCase {
    fn perform(&self, dto: CreateBookDto) -> Result<Book, ()>;
}

pub trait IGetBookUseCase {
    fn perform(&self, id: i32) -> Result<Option<Book>, ()>;
}

pub trait IUpdateBookUseCase {
    fn perform(&self, id: i32, dto: UpdateBookDto) -> Result<bool, ()>;
}

pub trait IDeleteBookUseCase {
    fn perform(&self, id: i32) -> Result<bool, ()>;
}

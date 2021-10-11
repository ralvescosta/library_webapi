use crate::errors::internal_error::InternalError;
use business::{
    dtos::{create_book_dto::CreateBookDto, update_book_dto::UpdateBookDto},
    entities::book::Book,
};

pub trait IBookRepository {
    fn create(&self, dto: CreateBookDto) -> Result<Book, InternalError>;
    fn get_by_id(&self, index: i32) -> Result<Option<Book>, InternalError>;
    fn update(&self, index: i32, dto: UpdateBookDto) -> Result<bool, InternalError>;
    fn delete_by_id(&self, index: i32) -> Result<bool, InternalError>;
}

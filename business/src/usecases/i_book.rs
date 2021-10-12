use crate::{dtos::create_book_dto::CreateBookDto, entities::book::Book};

pub trait ICreateBookUseCase {
    fn perform(&self, dto: CreateBookDto) -> Result<Book, ()>;
}

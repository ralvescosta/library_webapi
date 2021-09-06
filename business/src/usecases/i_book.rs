use crate::{dtos::create_book_dto::CreateBookDto, entities::book::Book};

pub trait IBookUseCase {
    fn perform(&self, dto: CreateBookDto) -> Book;
}

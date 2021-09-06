use business::{dtos::create_book_dto::CreateBookDto, entities::book::Book};

pub trait IBookRepository {
    fn create(&self, dto: CreateBookDto) -> Book;
}

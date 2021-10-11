use business::{
    dtos::{create_book_dto::CreateBookDto, update_book_dto::UpdateBookDto},
    entities::book::Book,
};

pub trait IBookRepository {
    fn create(&self, dto: CreateBookDto) -> Book;
    fn get_by_id(&self, index: i32) -> Option<Book>;
    fn update(&self, dto: UpdateBookDto) -> bool;
    fn delete_by_id(&self, index: i32) -> bool;
}

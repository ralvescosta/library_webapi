use application::interfaces::i_book_repository::IBookRepository;
use business::entities::book::Book;

pub struct BookRepository {}

impl IBookRepository for BookRepository {
    fn create(&self) -> Book {
        todo!()
    }
}

impl BookRepository {
    pub fn new() -> Box<BookRepository> {
        Box::new(BookRepository {})
    }
}

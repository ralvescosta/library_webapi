use application::IBookRepository;

pub struct BookRepository {}

impl IBookRepository for BookRepository {
    fn create(self) -> business::book::Book {
        todo!()
    }
}

use business::book::Book;

pub trait IBookRepository {
    fn create(self) -> Book;
}

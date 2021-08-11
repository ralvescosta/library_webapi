use business::entities::book::Book;

pub trait IBookRepository {
    fn create(&self) -> Book;
}

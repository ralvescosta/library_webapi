use business::entities::book::Book;

pub trait IBookRepository: Send {
    fn create(&self) -> Book;
}

use crate::interfaces::i_book_repository::IBookRepository;
use business::usecases::i_book::IBookUseCase;

pub struct BookUseCase {
    repository: Box<dyn IBookRepository>,
}

impl IBookUseCase for BookUseCase {
    fn perform(&self) {
        self.repository.create();
    }
}
impl BookUseCase {
    pub fn new(repository: Box<dyn IBookRepository>) -> BookUseCase {
        BookUseCase { repository }
    }
}

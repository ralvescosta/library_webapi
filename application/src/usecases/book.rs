use crate::interfaces::i_book_repository::IBookRepository;
use business::usecases::i_book::IBookUseCase;

pub struct BookUseCase {
    repository: dyn IBookRepository,
}

impl IBookUseCase for BookUseCase {
    fn perform(&self) {
        self.repository.create();
    }
}

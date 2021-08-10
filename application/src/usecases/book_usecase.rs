use business::IBookUsecase;

use crate::IBookRepository;

pub struct BookUsecase {
    repository: dyn IBookRepository,
}

impl IBookUsecase for BookUsecase {
    fn perform(&self) {
        let value = self.repository.create();
    }
}

use application::usecases::book::BookUseCase;
use business::usecases::i_book::IBookUseCase;
use infrastructure::repositories::book_repository::BookRepository;

fn main() {
    let repository = BookRepository::new();
    let usecase = BookUseCase::new(repository);
    usecase.perform();
}

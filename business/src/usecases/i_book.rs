use crate::dtos::create_book_dto::CreateBookDto;

pub trait IBookUseCase {
    fn perform(&self, dto: CreateBookDto);
}

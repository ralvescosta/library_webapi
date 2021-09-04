use crate::dtos::something_dto::SomethingDto;

pub trait ISomethingUseCase {
    fn perform(&self, dto: SomethingDto);
}

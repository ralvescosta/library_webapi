use business::usecases::i_something::ISomethingUseCase;
use business::dtos::something_dto::SomethingDto;

pub struct SomethingUseCase;

impl SomethingUseCase {
    pub fn new() -> SomethingUseCase {
        SomethingUseCase {}
    }
}

impl ISomethingUseCase for SomethingUseCase {
    fn perform(&self, dto: SomethingDto) {}
}

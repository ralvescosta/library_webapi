use business::dtos::something_dto::SomethingDto;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SomethingModel {
    first_name: String,
    last_name: String,
    alias: String,
    email: String,
}

impl SomethingModel {
    pub fn as_dto(&self) -> SomethingDto {
        SomethingDto {
            alias: self.alias.as_str(),
            email: self.email.as_str(),
            first_name: self.first_name.as_str(),
            last_name: self.last_name.as_str(),
        }
    }
}

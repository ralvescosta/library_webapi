use business::dtos::create_book_dto::CreateBookDto;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BookModel {
  pub title: String,
  pub subject: String,
  pub author: String,
  pub published_data: String,
  pub editor: String
}

impl BookModel {
  pub fn as_dto(&self) -> CreateBookDto {
    CreateBookDto{
      title: self.title.clone(),
      subject: self.subject.clone(),
      author: self.author.clone(),
      published_data: self.published_data.clone(),
      editor: self.editor.clone(),
    }
  }
}
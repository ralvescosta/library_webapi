use business::{
    dtos::{create_book_dto::CreateBookDto, update_book_dto::UpdateBookDto},
    entities::book::Book,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateBookModel {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_data: Option<String>,
    pub editor: String,
}

impl CreateBookModel {
    pub fn to_create_book_dto(&self) -> CreateBookDto {
        CreateBookDto {
            title: self.title.clone(),
            subject: self.subject.clone(),
            author: self.author.clone(),
            published_data: self.published_data.clone(),
            editor: self.editor.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct ResponseCreateBookModel {
    pub id: i32,
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_data: Option<String>,
    pub editor: String,
    pub created_at: String,
}

impl ResponseCreateBookModel {
    pub fn from_book(book: Book) -> ResponseCreateBookModel {
        ResponseCreateBookModel {
            id: book.id,
            author: book.author,
            editor: book.editor,
            published_data: book.published_date,
            subject: book.subject,
            title: book.title,
            created_at: book.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateBookModel {
    pub title: Option<String>,
    pub subject: Option<String>,
    pub author: Option<String>,
    pub editor: Option<String>,
}

impl UpdateBookModel {
    pub fn to_update_book_dto(&self) -> UpdateBookDto {
        UpdateBookDto {
            title: self.title.clone(),
            subject: self.subject.clone(),
            author: self.author.clone(),
            editor: self.editor.clone(),
        }
    }
}

use business::{dtos::create_book_dto::CreateBookDto, entities::book::Book};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BookModel {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_data: Option<String>,
    pub editor: String,
}

impl BookModel {
    pub fn to_create_book_dto(&self) -> CreateBookDto {
        CreateBookDto {
            title: self.title.clone(),
            subject: self.subject.clone(),
            author: self.author.clone(),
            published_data: self.published_data.clone(),
            editor: self.editor.clone(),
        }
    }

    pub fn from_book(book: Book) -> BookModel {
        BookModel {
            author: book.author,
            editor: book.editor,
            published_data: book.published_date,
            subject: book.subject,
            title: book.title,
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

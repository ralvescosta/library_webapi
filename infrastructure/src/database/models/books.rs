use crate::schema::books;
use business::{dtos::create_book_dto::CreateBookDto, entities::book::Book};
use chrono::{DateTime, Utc};

#[derive(Insertable)]
#[table_name = "books"]
pub struct CreateBookModel {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_date: Option<DateTime<Utc>>,
    pub editor: String,
}

impl CreateBookModel {
    pub fn from_create_book_dto(dto: CreateBookDto) -> CreateBookModel {
        CreateBookModel {
            title: dto.title,
            subject: dto.subject,
            author: dto.author,
            published_date: None,
            editor: dto.editor,
        }
    }
}

#[derive(Debug, Queryable)]
pub struct BookModel {
    pub id: i32,
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_date: Option<DateTime<Utc>>,
    pub editor: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl BookModel {
    pub fn to_book(self) -> Book {
        let published_date = match self.published_date {
            Some(date) => Some(date.to_string()),
            None => None,
        };
        let deleted_at = match self.deleted_at {
            Some(date) => Some(date.to_string()),
            None => None,
        };
        Book {
            id: self.id,
            title: self.title,
            subject: self.subject,
            author: self.author,
            published_date,
            editor: self.editor,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
            deleted_at,
        }
    }
}

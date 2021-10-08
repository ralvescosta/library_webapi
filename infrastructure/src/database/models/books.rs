use crate::schema::books;
use chrono::{DateTime, Utc};

#[derive(Insertable)]
#[table_name = "books"]
pub struct CreateBookModel {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_date: Option<DateTime<Utc>>,
    pub editor: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

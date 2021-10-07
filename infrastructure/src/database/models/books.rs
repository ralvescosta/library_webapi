use crate::schema::books;

#[derive(Insertable)]
#[table_name = "books"]
pub struct CreateBookModel {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_date: String,
    pub editor: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

#[derive(Debug, Queryable)]
pub struct BookModel {
    pub id: i32,
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_date: String,
    pub editor: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

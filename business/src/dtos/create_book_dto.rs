#[derive(Clone)]
pub struct CreateBookDto {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub published_data: String,
    pub editor: String,
}

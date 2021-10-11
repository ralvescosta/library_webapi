use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

use application::interfaces::{i_book_repository::IBookRepository, i_logger::ILogger};
use business::dtos::{create_book_dto::CreateBookDto, update_book_dto::UpdateBookDto};
use business::entities::book::Book;

use crate::database::models::books::{BookModel, CreateBookModel, UpdateBookModel};

pub struct BookRepository {
    _logger: Arc<dyn ILogger>,
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl IBookRepository for BookRepository {
    fn create(&self, dto: CreateBookDto) -> Book {
        use crate::schema::books;

        let connection = self.pool.get().unwrap();

        let result: BookModel = diesel::insert_into(books::table)
            .values(CreateBookModel::from_create_book_dto(dto))
            .get_result(&connection)
            .expect("[BookRepository::create - Error]");

        result.to_book()
    }

    fn get_by_id(&self, index: i32) -> Option<Book> {
        use crate::schema::books::dsl::*;

        let connection = self.pool.get().unwrap();

        let books_model: Vec<BookModel> = books
            .filter(id.eq(index))
            .load(&connection)
            .expect("[BookRepository::get_by_id - Error]");

        if let Some(book) = books_model.first() {
            return Some(book.to_book());
        }

        return None;
    }

    fn update(&self, index: i32, dto: UpdateBookDto) -> bool {
        use crate::schema::books::dsl::books;

        let connection = self.pool.get().unwrap();

        diesel::update(books.find(index))
            .set(UpdateBookModel::from_update_book_dto(dto))
            .execute(&connection)
            .expect("[BookRepository::update - Error]");

        true
    }

    fn delete_by_id(&self, index: i32) -> bool {
        use crate::schema::books::dsl::{books, id};

        let connection = self.pool.get().unwrap();

        diesel::delete(books.filter(id.eq(index)))
            .execute(&connection)
            .expect("[BookRepository::delete_by_id - Error]");

        true
    }
}

impl BookRepository {
    pub fn new(
        logger: Arc<dyn ILogger>,
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    ) -> impl IBookRepository {
        BookRepository {
            _logger: logger,
            pool,
        }
    }
}

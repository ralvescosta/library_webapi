use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

use application::interfaces::{i_book_repository::IBookRepository, i_logger::ILogger};
use business::dtos::{create_book_dto::CreateBookDto, update_book_dto::UpdateBookDto};
use business::entities::book::Book;

use crate::database::models::books::{BookModel, CreateBookModel, UpdateBookModel};
use application::errors::internal_error::InternalError;

pub struct BookRepository {
    logger: Arc<dyn ILogger>,
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl IBookRepository for BookRepository {
    fn create(&self, dto: CreateBookDto) -> Result<Book, InternalError> {
        use crate::schema::books;

        let connection = self.pool.get().unwrap();

        if let Ok(result) = diesel::insert_into(books::table)
            .values(CreateBookModel::from_create_book_dto(dto))
            .get_result::<BookModel>(&connection)
        {
            return Ok(result.to_book());
        }

        self.logger.error(
            "[BookRepository::create]",
            "Some error occur while creating the book",
        );
        return Err(InternalError::Default);
    }

    fn get_by_id(&self, index: i32) -> Result<Option<Book>, InternalError> {
        use crate::schema::books::dsl::*;

        let connection = self.pool.get().unwrap();

        if let Ok(book_model) = books.filter(id.eq(index)).first::<BookModel>(&connection) {
            return Ok(Some(book_model.to_book()));
        }

        Ok(None)
    }

    fn update(&self, index: i32, dto: UpdateBookDto) -> Result<bool, InternalError> {
        use crate::schema::books::dsl::books;

        let connection = self.pool.get().unwrap();

        if let Ok(_) = diesel::update(books.find(index))
            .set(UpdateBookModel::from_update_book_dto(dto))
            .execute(&connection)
        {
            return Ok(true);
        }

        self.logger.error(
            "[BookRepository::create]",
            "Some error occur while updating the book",
        );

        Err(InternalError::Default)
    }

    fn delete_by_id(&self, index: i32) -> Result<bool, InternalError> {
        use crate::schema::books::dsl::{books, id};

        let connection = self.pool.get().unwrap();

        if let Ok(_) = diesel::delete(books.filter(id.eq(index))).execute(&connection) {
            return Ok(true);
        }

        self.logger.error(
            "[BookRepository::create]",
            "Some error occur while deleting the book",
        );

        Err(InternalError::Default)
    }
}

impl BookRepository {
    pub fn new(
        logger: Arc<dyn ILogger>,
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    ) -> impl IBookRepository {
        BookRepository { logger, pool }
    }
}

use std::env;

use diesel::{r2d2::ConnectionManager, PgConnection};

pub fn create_connection_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let connection_string =
        env::var("DATABASE_URL").expect(".env.ENVIRONMENTS must contains DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    r2d2::Pool::builder().max_size(10).build(manager).unwrap()
}

use rocket_db_pools::Database;

pub mod crates;
pub mod index;
pub mod rustaceans;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);
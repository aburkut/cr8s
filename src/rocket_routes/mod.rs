use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket_db_pools::Database;
use std::error::Error;

pub mod crates;
pub mod index;
pub mod rustaceans;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

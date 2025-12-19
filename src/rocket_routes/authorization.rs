use crate::auth::{Credentials, authorize_user};
use crate::repositories::UserRepository;
use crate::rocket_routes::{CacheConn, DbConn, server_error};
use crate::schema::users::password;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Json, Value, json};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>,
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_username(&mut db, &credentials.username)
        .await
        .map_err(|e| match e {
            DieselError::NotFound => Custom(Status::Unauthorized, json!("User not found")),
            _ => server_error(e.into()),
        })?;

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache
        .set_ex::<String, i32, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 6)
        .await
        .map_err(|e| server_error(e.into()))?;

    Ok(json!({
        "token": session_id
    }))
}

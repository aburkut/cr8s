use crate::auth;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{RoleRepository, UserRepository};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use std::str::FromStr;

async fn load_db_connection() -> AsyncPgConnection {
    let database_url =
        std::env::var("DATABASE_URL").expect("Cannot retrieve  DB url from environment variable");

    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to database")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let password_hash = auth::hash_password(password).unwrap();

    let new_user = NewUser {
        username,
        password: password_hash.to_string(),
    };
    let role_enums = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v.as_str()).unwrap())
        .collect();

    let user = UserRepository::create(&mut c, new_user, role_enums)
        .await
        .unwrap();

    println!("Created new user: {:?}", user);

    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("Found roles {:?}", roles);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut c).await.unwrap();

    for user in users {
        println!("{:?}", user);
    }
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
    UserRepository::delete(&mut c, id).await.unwrap();
}

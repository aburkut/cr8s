use crate::auth;
use crate::mail::HtmlMailer;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{CrateRepository, RoleRepository, UserRepository};
use chrono::{Datelike, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use std::str::FromStr;
use tera::{Context, Tera};

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").expect("Cannot load templtate engine")
}

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

pub async fn digest_send(email: String, hours_since: i32) {
    let mut c = load_db_connection().await;
    let tera = load_template_engine();

    let crates = CrateRepository::find_since(&mut c, hours_since)
        .await
        .unwrap();

    println!("crates.len() {}", crates.len());
    if crates.len() > 0 {
        println!("Sending digest for {} crates", crates.len());
        let year = Utc::now().year();
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);

        let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST env variable is not set");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME env variable is not set");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable is not set");

        let mailer = HtmlMailer {
            template_engine: tera,
            smtp_host,
            smtp_username,
            smtp_password,
        };

        match mailer.send(email, "email/digest.html", context) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => eprintln!("Could not send email: {:?}", e),
        }
    }
}

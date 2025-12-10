use rocket_db_pools::Database;
mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                /* Rustcean routes */
                rocket_routes::rustaceans::get_rustaceans,
                rocket_routes::rustaceans::get_rustacean,
                rocket_routes::rustaceans::create_rustacean,
                rocket_routes::rustaceans::update_rustacean,
                rocket_routes::rustaceans::delete_rustacean,
                /* Crate routes */
                rocket_routes::crates::get_crates,
                rocket_routes::crates::get_crate,
                rocket_routes::crates::create_crate,
                rocket_routes::crates::update_crate,
                rocket_routes::crates::delete_crate,
                /* Index routes */
                rocket_routes::index::get_index,
                rocket_routes::index::get_hello,
            ],
        )
        .attach(rocket_routes::DbConn::init())
        .launch()
        .await;
}

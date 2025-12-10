#[rocket::get("/")]
pub fn get_index() -> String {
    String::from("Hello, I'm index page")
}

#[rocket::get("/hello")]
pub fn get_hello() -> String {
    String::from("Hello, world!")
}

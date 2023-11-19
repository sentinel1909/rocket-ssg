// src/lib/routes/index.rs

// dependencies
use rocket::{get, response::content::RawHtml};

// index route handler
#[get("/")]
pub fn index() -> RawHtml<String> {
    RawHtml(String::from("<h1>Hello, world!</h1>"))
}

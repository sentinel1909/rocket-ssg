// src/lib/routes/index.rs

// dependencies
use rocket::{get, response::content::RawHtml};
use crate::parser::parser;

// index route handler
#[get("/")]
pub async fn index() -> RawHtml<String> {
    parser().await
}

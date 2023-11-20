// src/lib/routes/index.rs

// dependencies
use crate::parser::parser;
use rocket::{get, response::content::RawHtml};

// index route handler
#[get("/<path>")]
pub async fn index(path: &str) -> RawHtml<String> {
    parser(path).await
}

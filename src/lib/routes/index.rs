// src/lib/routes/index.rs

// dependencies
use crate::parser::parser;
use rocket::{get, response::content::RawHtml};

// index route handler
#[get("/")]
pub async fn index() -> RawHtml<String> {
    parser().await
}

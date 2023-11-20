// src/lib/routes/styles.rs

// dependencies
use rocket::fs::{NamedFile, relative};
use rocket::get;
use rocket::response::status::NotFound;
use std::path::{Path, PathBuf};

// route for styles.css, returns the file if it exists
#[get("/<file..>")]
pub async fn styles(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new(relative!("templates/")).join(file);
    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

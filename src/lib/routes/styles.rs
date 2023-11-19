// src/lib/routes/styles.rs

// dependencies
use rocket::get;

// styles route
#[get("/styles.css")]
pub async fn styles() -> String {
    let file_name = String::from("templates/styles.css");

    let css_input = match tokio::fs::read(file_name).await {
        Ok(res) => res,
        Err(_) => return String::from("/* Error reading css input file */"),
    };

    let styles_output = std::str::from_utf8(&css_input).unwrap();

    styles_output.to_string()
}

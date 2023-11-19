// src/lib/routes/catchers.rs

// dependencies
use rocket::catch;
use rocket::Request;
use rocket::response::content::RawHtml;

// 404 handler
#[catch(404)]
pub fn not_found(req: &Request) -> RawHtml<String> {
    RawHtml(format!(
        "<h1>Oh no! We couldn't find the requested path '{}'</h1>",
        req.uri()
    ))
}

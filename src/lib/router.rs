// src/lib/router.rs

// dependencies
use crate::routes::catchers;
use crate::routes::index::index;
use crate::routes::styles::styles;
use rocket::catchers;
use rocket::routes;
use rocket::{Build, Rocket};

// function to create a Rocket instance
pub fn create() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![styles])
        .register("/", catchers![catchers::not_found])
}

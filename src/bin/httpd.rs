// src/bin/httpd.rs

// dependencies
use rocket_ssg::router::create;

// shuttle_runtime::main is a procedural macro that generates a main function
#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = create();

    Ok(rocket.into())
}

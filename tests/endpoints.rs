// tests/httpd.rs

// integration test for the route at /
#[cfg(test)]
mod endpoint_tests {
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket_ssg::router::create;

    #[test]
    fn test_index() {
        let client = Client::tracked(create()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            "<head><link rel=\"stylesheet\" type=\"text/css\" href=\"/styles.css\"></head><body><h1>Rocket + Markdown!</h1>\n<p>This file is written in markdown, parsed to HTML, and served up by Rocket.</p>\n</body>".to_string()
        );
    }

    #[test]
    fn test_not_found() {
        let client = Client::tracked(create()).expect("valid rocket instance");
        let response = client.get("/not-found").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.into_string().unwrap(),
            "No such file or directory (os error 2)"
        );
    }
}

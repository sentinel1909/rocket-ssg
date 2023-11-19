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
        assert_eq!(response.into_string().unwrap(), "<h1>Hello, World!</h1>\n");
    }

    #[test]
    fn test_index_error() {
        let client = Client::tracked(create()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            "<h1>Error reading markdown input file</h1>"
        );
    }

    #[test]
    fn test_not_found() {
        let client = Client::tracked(create()).expect("valid rocket instance");
        let response = client.get("/not-found").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.into_string().unwrap(),
            "<h1>Oh no! We couldn't find the requested path '/not-found'</h1>"
        );
    }
}

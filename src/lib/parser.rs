// src/lib/parser.rs

// parser function, takes a markdown file as input, returns the parsed html

// dependencies
use rocket::response::content::RawHtml;

pub async fn parser() -> RawHtml<String> {
    let file_name = String::from("templates/hello_world.md");

    let markdown_input = match tokio::fs::read(file_name).await {
        Ok(res) => res,
        Err(_) => return RawHtml(String::from("<h1>Error reading markdown input file</h1>")),
    };

    let string_output = std::str::from_utf8(&markdown_input).unwrap();

    let mut html_output = String::new();
    html_output.push_str(
        r#"<head><link rel="stylesheet" type="text/css" href="/styles.css"></head><body>"#,
    );
    let parser = pulldown_cmark::Parser::new(string_output);
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output.push_str("</body>");
    RawHtml(html_output)
}

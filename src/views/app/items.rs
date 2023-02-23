use std::fs;

use actix_web::HttpResponse;

pub async fn items() -> HttpResponse {
    let html_data =
        fs::read_to_string("./static/home.html").expect("Unable to open static html file");
    let js_data = fs::read_to_string("./dynamic/home.js").expect("Unable to open js file");

    let html_data = html_data.replace("{{JAVASCRIPT}}", &js_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}

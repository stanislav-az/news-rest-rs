use std::fs;

use actix_web::HttpResponse;

pub async fn items() -> HttpResponse {
    let html_data =
        fs::read_to_string("./static/home.html").expect("Unable to open static html file");
    let js_data = fs::read_to_string("./dynamic/home.js").expect("Unable to open js file");
    let base_css = fs::read_to_string("./styles/base.css").expect("Unable to open base css file");
    let home_css = fs::read_to_string("./styles/home.css").expect("Unable to open home css file");

    let html_data = html_data.replace("{{JAVASCRIPT}}", &js_data);
    let html_data = html_data.replace("{{BASE_CSS}}", &base_css);
    let html_data = html_data.replace("{{CSS}}", &home_css);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}

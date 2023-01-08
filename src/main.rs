use std::{collections::HashMap, fs, sync::Arc, thread, time::Duration};

use news_rest_rs::service::{
    logger::{LogLevel, Logger, LoggerSettings},
    webserver::{run_server, Method, Request, Response},
};

fn main() {
    let application = |req: Request| {
        if req.uri.is_empty() && req.method == Method::GET {
            let body = fs::read_to_string("static/main_page.html").unwrap();

            return Response {
                status: 200,
                status_text: String::from("OK"),
                headers: HashMap::new(),
                body,
            };
        }
        if req.uri == vec!["sleep"] && req.method == Method::GET {
            thread::sleep(Duration::from_secs(5));

            return Response {
                status: 200,
                status_text: String::from("OK"),
                headers: HashMap::new(),
                body: String::from("Hello from awake rust server!"),
            };
        }

        let body = fs::read_to_string("static/404.html").unwrap();
        Response {
            status: 404,
            status_text: String::from("NOT FOUND"),
            headers: HashMap::new(),
            body,
        }
    };

    let application = Arc::new(application);
    run_server(application)
}

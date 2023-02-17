use std::{collections::HashMap, fs, thread, time::Duration};

use news_rest_rs::service::{
    logger::{LogLevel, Logger, LoggerSettings},
    webserver::{run_server, ContentType, Method, Request, Response, ResponseBody},
};

fn main() {
    let logger_settings = LoggerSettings {
        log_to_file: Some(String::from("logs/journal.log")),
        log_to_stderr: true,
        log_level_starting_from: LogLevel::Debug,
    };
    let mut logger = Logger::initialize(&logger_settings);
    logger.log_debug("Logger started");

    let mut logger = logger.clone();
    let application = move |req: Request| {
        logger.log_info(&format!("App received request: {:?}", req));

        if req.uri.is_empty() && req.method == Method::GET {
            let body = fs::read_to_string("static/main_page.html").unwrap();

            return Response {
                status: 200,
                status_text: String::from("OK"),
                headers: HashMap::new(),
                body: Some(ResponseBody {
                    content: body,
                    content_type: ContentType::HTML,
                }),
            };
        }
        if req.uri == vec!["sleep"] && req.method == Method::GET {
            thread::sleep(Duration::from_secs(5));

            return Response {
                status: 200,
                status_text: String::from("OK"),
                headers: HashMap::new(),
                body: Some(ResponseBody {
                    content: String::from("Hello from awake rust server!"),
                    content_type: ContentType::PlainText,
                }),
            };
        }

        let body = fs::read_to_string("static/404.html").unwrap();
        Response {
            status: 404,
            status_text: String::from("NOT FOUND"),
            headers: HashMap::new(),
            body: Some(ResponseBody {
                content: body,
                content_type: ContentType::HTML,
            }),
        }
    };

    run_server(application)
}

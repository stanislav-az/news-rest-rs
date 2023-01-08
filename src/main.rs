use std::{thread, time::Duration, collections::HashMap, sync::Arc};

use news_rest_rs::service::{
    logger::{LogLevel, Logger, LoggerSettings},
    webserver::{run_server, Request, Response},
};

fn main() {
    let application = |req: Request| Response {
        status: 200,
        status_text: String::from("OK"),
        headers: HashMap::new(),
        body: String::from("hello from server!"),
    };
    let application = Arc::new(application);
    run_server(application)
}

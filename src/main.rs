use std::{thread, time::Duration};

use news_rest_rs::service::{
    logger::{LogLevel, Logger, LoggerSettings},
    webserver::run_server,
};

fn main() {
    run_server()
}

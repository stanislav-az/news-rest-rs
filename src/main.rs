use std::{thread, time::Duration};

use news_rest_rs::service::{
    logger::{LogLevel, Logger, LoggerSettings},
    webserver::run_server,
};

fn main() {
    let settings = LoggerSettings {
        log_to_file: Some(String::from("logs/journal.log")),
        log_to_stderr: true,
        log_level_starting_from: LogLevel::Debug,
    };
    let mut logger = Logger::initialize(&settings);
    logger.log_debug("Logger started");
    thread::sleep(Duration::from_secs(1));
    let mut thread_logger = logger.clone();
    let thread = thread::spawn(move || {
        thread_logger.log_info("Hello from another threeeaad!");
        thread::sleep(Duration::from_secs(1));
    });
    thread.join().unwrap();
}

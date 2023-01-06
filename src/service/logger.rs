use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
pub struct Logger {
    log_conf: LoggerSettings,
    log_chan: Option<mpsc::Sender<LoggerMsg>>,
    log_thread: Option<thread::JoinHandle<()>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggerMsg {
    pub text: String,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct LoggerSettings {
    pub log_to_file: Option<String>,
    pub log_to_stderr: bool,
    pub log_level_starting_from: LogLevel,
}

impl Drop for Logger {
    fn drop(&mut self) {
        // TODO rm
        dbg!("Shutting down logger...");
        drop(self.log_chan.take());

        if let Some(thread) = self.log_thread.take() {
            thread.join().unwrap();
        }
    }
}

impl Logger {
    pub fn initialize(config: &LoggerSettings) -> Self {
        let mut file = if let Some(path) = &config.log_to_file {
            let f = File::options()
                .append(true)
                .create(true)
                .open(path)
                .expect(&format!("Could not open log file on path {}", path));
            Some(f)
        } else {
            None
        };
        let log_to_stderr = config.log_to_stderr;
        let (sender, receiver) = mpsc::channel();
        let log_thread = thread::spawn(move || loop {
            let try_recv = receiver.recv();
            match try_recv {
                Ok(msg) => {
                    let json =
                        serde_json::to_string(&msg).expect("Could not serialize log message");
                    if log_to_stderr {
                        eprintln!("{}", &json);
                    }
                    if let Some(file) = &mut file {
                        writeln!(file, "{}", &json).expect("Could not append to log file");
                    }
                }
                Err(_) => {
                    // TODO rm
                    dbg!("Logger disconnected, shutting down...");
                    break;
                }
            }
        });
        Logger {
            log_conf: config.clone(),
            log_chan: Some(sender),
            log_thread: Some(log_thread),
        }
    }

    pub fn disable_logs() -> Self {
        Logger {
            log_conf: LoggerSettings {
                log_to_file: None,
                log_to_stderr: false,
                log_level_starting_from: LogLevel::Error,
            },
            log_chan: None,
            log_thread: None,
        }
    }

    pub fn are_logs_enabled_for_level(&self, level: LogLevel) -> bool {
        level >= self.log_conf.log_level_starting_from
            && (self.log_conf.log_to_stderr || self.log_conf.log_to_file.is_some())
    }

    pub fn log(&mut self, level: LogLevel, msg_text: &str) {
        if self.are_logs_enabled_for_level(level) {
            let msg = LoggerMsg {
                text: msg_text.to_string(),
                level,
                timestamp: Utc::now(),
            };
            self.log_chan.as_ref().unwrap().send(msg).unwrap();
        }
    }

    pub fn log_debug(&mut self, msg_text: &str) {
        self.log(LogLevel::Debug, msg_text)
    }
    pub fn log_info(&mut self, msg_text: &str) {
        self.log(LogLevel::Info, msg_text)
    }
    pub fn log_warn(&mut self, msg_text: &str) {
        self.log(LogLevel::Warn, msg_text)
    }
    pub fn log_error(&mut self, msg_text: &str) {
        self.log(LogLevel::Error, msg_text)
    }
}

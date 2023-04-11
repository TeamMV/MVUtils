use std::string::ToString;
use chrono::{Datelike, Timelike, Utc};
use crate::logger::LogLevel::{DEBUG, ERROR, INFO, WARN};

pub struct Logger<F> {
    action: F,
    log_fmt: String
}

impl<F> Logger<F>
where F: Fn(LogLevel, String) {
    pub fn new(action: F) -> Self {
        Logger {
            action,
            log_fmt: "[${date}] <${level}> ${message}".to_string()
        }
    }

    fn get_date(&self) -> String {
        let now = Utc::now();
        format!("{}-{}-{} at {}:{}:{}:{}", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second(), now.nanosecond())
    }

    fn build_log(&self, lvl: LogLevel, msg: String) -> String {
        self.log_fmt.replace("${date}", self.get_date().as_str()).replace("${level}", lvl.to_string().as_str()).replace("${message}", msg.as_str())
    }

    pub fn debug(&self, message: String) {
        (self.action)(DEBUG, self.build_log(DEBUG, message));
    }

    pub fn info(&self, message: String) {
        (self.action)(INFO, self.build_log(INFO, message));
    }

    pub fn warn(&self, message: String) {
        (self.action)(WARN, self.build_log(WARN, message));
    }

    pub fn error(&self, message: String) {
        (self.action)(ERROR, self.build_log(ERROR, message));
    }

    pub fn set_action(&mut self, action: F) {
        self.action = action;
    }

    pub fn set_log_fmt(&mut self, log_fmt: String) {
        self.log_fmt = log_fmt;
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            DEBUG => "debug",
            INFO => "info",
            WARN => "warn",
            ERROR => "error",
        }.to_string()
    }
}
// use log::{error, info, warn};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::sync::Mutex;
use tauri::Manager;

#[derive(Debug, PartialEq, Serialize, Clone)]
struct LogEntry {
    message: Option<String>,
    level: Option<String>,
    timestamp: Option<String>,
}

pub struct Logger {
    pub app_handle: Option<Box<tauri::AppHandle>>,
}

pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger { app_handle: None }));

impl Logger {
    pub fn log_message(&mut self, log: &LogEntry) {
        if let Some(ref mut app) = self.app_handle {
            app.emit_all("log_event", log).unwrap();
        }
    }

    pub fn danger(&mut self, message: &str) {
        let timestamp = chrono::Local::now().to_rfc2822();
        let log_entry = LogEntry {
            message: Some(message.to_string()),
            level: Some("danger".to_string()),
            timestamp: Some(timestamp),
        };

        self.log_message(&log_entry);
    }

    pub fn info(&mut self, message: &str) {
        let timestamp = chrono::Local::now().to_rfc2822();
        let log_entry = LogEntry {
            message: Some(message.to_string()),
            level: Some("info".to_string()),
            timestamp: Some(timestamp),
        };

        self.log_message(&log_entry);
    }

    pub fn warning(&mut self, message: &str) {
        let timestamp = chrono::Local::now().to_rfc2822();
        let log_entry = LogEntry {
            message: Some(message.to_string()),
            level: Some("warning".to_string()),
            timestamp: Some(timestamp),
        };

        self.log_message(&log_entry);
    }
}

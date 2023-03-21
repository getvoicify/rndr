use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use crate::utils::logger::Logger;
use chrono::prelude::*;

pub trait FileLoggerPath {
    fn set_log_file_path(&mut self, file_path: &str);
}

#[derive(Clone)]
pub struct FileLogger {
    pub file_path: String,
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        let log_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();
        let mut writer = BufWriter::new(log_file);

        let dt = Local::now();
        let formatted = dt.format("%Y-%m-%dT%H:%M:%S%.3f%:z");
        writeln!(writer, "{} {}", formatted, message).expect("Failed to write line");
        println!("{} {}", formatted, message);
    }
}

impl FileLoggerPath for FileLogger {
    fn set_log_file_path(&mut self, file_path: &str) {
        self.file_path = file_path.to_string();
    }
}
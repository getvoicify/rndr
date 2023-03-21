use crate::utils::logger::Logger;

pub struct SentryLogger;

pub trait SentryLogError {
    fn log_error(&self, error: &str);
}

impl Logger for SentryLogger {
    fn log(&self, message: &str) {
        sentry::capture_message(message, sentry::Level::Info);
    }
}

impl SentryLogError for SentryLogger {
    fn log_error(&self, error: &str) {
        sentry::capture_message(error, sentry::Level::Error);
    }
}
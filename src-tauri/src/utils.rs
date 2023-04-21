pub mod logger;
pub mod sentry_logger;
pub mod file_logger;
pub mod read_file_to_text_string;
pub mod error;
pub mod aws_credentials;
pub mod aws_client_factory;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub total_count: usize,
}
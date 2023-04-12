pub mod logger;
pub mod sentry_logger;
pub mod file_logger;
pub mod read_file_to_text_string;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub total_count: usize,
}
pub mod logger;
pub mod sentry_logger;
pub mod file_logger;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub total_count: usize,
}
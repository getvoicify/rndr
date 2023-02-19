#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub total_count: usize,
}
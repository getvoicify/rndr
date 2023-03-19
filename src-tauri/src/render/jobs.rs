use csv::Reader;

use crate::utils::PaginatedData;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Job {
    id: String,
    source_blend_path: String,
    description: String,
    package: String,
    additional_file_count: String,
    key: String,
    scene: String,
    xres: String,
    yres: String,
    samples: String,
    percentage: String,
    startframe: String,
    endframe: String,
    step: String,
    is_uploaded: String,
    status: String,
    cloudid: String,
    error: String,
    #[serde(rename = "startedAt")]
    started_at: String,
    #[serde(rename = "stoppedAt")]
    stopped_at: String,
    use_large_disk: String,
    parent: String,
}

#[tauri::command]
pub fn parse_csv(
    file_path: &str,
    order: Option<&str>,
    page: u32,
    per_page: u32,
) -> PaginatedData<Job> {
    println!("Parsing CSV file: {}", file_path);
    let mut data = vec![];
    let mut rdr = match Reader::from_path(file_path) {
        Ok(rdr) => rdr,
        Err(e) => {
            sentry::capture_error(&e);
            println!("Error: {}", e);
            return PaginatedData {
                data,
                total_count: 0,
            };
        }
    };

    for result in rdr.deserialize() {
        let record: Job = match result {
            Ok(record) => record,
            Err(e) => {
                sentry::capture_error(&e);
                println!("Error: {}", e);
                return PaginatedData {
                    data,
                    total_count: 0,
                };
            }
        };
        data.push(record);
    }

    match order {
        Some("asc") => data.sort_by(|a, b| a.id.cmp(&b.id)),
        Some("desc") => data.sort_by(|a, b| b.id.cmp(&a.id)),
        _ => (),
    };

    let total_count = data.len();
    let start = (page - 1) as usize * per_page as usize;
    let end = start as usize + per_page as usize;
    let end = if end > total_count { total_count } else { end };

    PaginatedData {
        data: data[start..end].to_vec(),
        total_count,
    }
}
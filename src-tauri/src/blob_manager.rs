use std::path::Path;

use aws_sdk_s3 as s3;
use aws_smithy_http::byte_stream::ByteStream;

use crate::utils::file_logger::FileLogger;
use crate::utils::logger::Logger;

trait Loggable {
    fn log(&self, message: &str);
}

pub struct BlobManager {
    pub s3_client: Option<s3::Client>,
    pub file_logger: Option<FileLogger>,
    pub render_path: Option<String>,
    pub work_dir: Option<String>,
}

impl Loggable for BlobManager {
    fn log(&self, message: &str) {
        if let Some(logger) = &self.file_logger {
            logger.log(message);
        }
    }
}

impl BlobManager {
    async fn upload(&self, path: &str, bucket: &str) -> Result<String, String> {
        self.log(&*format!("[Rust]: Uploading {} to S3", path));
        if let Some(s3_client) = &self.s3_client {
            let file_path = Path::new(path);

            if !file_path.exists() {
                self.log(&*format!("[Rust]: File {} does not exist", path));
                return Err("File does not exist".to_string());
            }

            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let body = ByteStream::from_path(file_path).await;
            let body = match body {
                Ok(s) => s,
                Err(err) => {
                    self.log(&*format!("[RUST]: Error - An error occurred while parsing the file."));
                    return Err(err.to_string());
                }
            };
            let resp = s3_client
                .put_object()
                .bucket(bucket)
                .key(file_name)
                .body(body)
                .send()
                .await;
            self.log(&*format!("[Rust]: Upload response: {:?}", resp));
            return Ok("File uploaded".parse().unwrap());
        }
        self.log("[RUST]: Error - S3 client not initialized");
        Err("S3 client not initialized".to_string())
    }
}
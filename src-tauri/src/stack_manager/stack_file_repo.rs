use tauri::{AppHandle, State, Wry};
use git2::Repository;
use crate::utils::file_logger::FileLogger;
use crate::utils::logger::Logger;

#[tauri::command]
pub fn has_stack_file_repo(handle: AppHandle<Wry>, logger: State<FileLogger>) -> bool {
    logger.log("[RUST]: Checking stack repo");
    match handle.path_resolver().app_data_dir().unwrap().join(".config").join(".dep_repo").exists() {
        true => {
            logger.log("[RUST]: Stack repo exists");
            true
        }
        false => {
            logger.log("[RUST]: Stack repo does not exists");
            false
        }
    }
}

#[tauri::command]
pub async fn create_stack_file_repo(handle: AppHandle<Wry>, logger: State<'_, FileLogger>) -> Result<String, String> {
    logger.log("[RUST]: Cloning stack repo");
    let url = "https://github.com/petetaxi-test/AwsBatchBlender";
    let path = handle.path_resolver().app_data_dir().unwrap();
    let path = path.join(".config").join(".dep_repo");

    match Repository::clone(url, path) {
        Ok(_) => Ok("Repository cloned successfully!".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
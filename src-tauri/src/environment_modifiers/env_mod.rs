use std::{env, fs};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use tauri::{App, State, Wry};
use tauri::api::path::home_dir;

use crate::utils::file_logger::FileLogger;
use crate::utils::logger::Logger;

fn write_hash_to_file(filename: &str, file_map: HashMap<String, String>) {
    let mut file = BufWriter::new(
        File::create(filename).expect("Failed to create file"),
    );

    for (key, value) in file_map {
        writeln!(&mut file, "{}={}", key, value).expect("Failed to write line");
    }
}

fn parse_file(filename: &str) -> HashMap<String, String> {
    let mut file_map = HashMap::new();

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return file_map,
    };
    // iterate over lines
    for line in BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        let mut split = line.splitn(2, '=');
        let key = split.next().expect("Failed to read key");
        let value = split.next().expect("Failed to read value");
        file_map.insert(key.to_string(), value.to_string());
    }
    file_map
}

#[tauri::command]
pub fn add_or_update_env_var(file_name: &str, key: &str, value: &str) {
    let mut file_map = parse_file(file_name);
    file_map.insert(key.to_string(), value.to_string());
    write_hash_to_file(file_name, file_map);
    match env::var(key) {
        Ok(val) => if val != value {
            env::set_var(key, value);
        },
        Err(_e) => env::set_var(key, value),
    };
}

pub fn run_bootstrap(file_name: &str, app: &mut App<Wry>, logger: &impl Logger) {
    bootstrap_env(file_name, logger);
    let app_data_dir = app.path_resolver().app_data_dir().unwrap();
    let app_data_dir = app_data_dir.to_str().unwrap();
    match Path::new(app_data_dir).exists() {
        true => logger.log(&*format!("[RUST]: {} exists", app_data_dir)),
        false => {
            logger.log(&*format!("[RUST]: {} does not exist, creating...", app_data_dir));
            fs::create_dir_all(app_data_dir).unwrap();
        }
    }
}

pub fn bootstrap_env(file_name: &str, logger: &impl Logger) {
    logger.log(&*format!("[RUST]: Bootstrapping environment variables to {}", file_name));
    let var_hashmap = parse_file(file_name);

    for (key, value) in var_hashmap {
        match env::var(&key) {
            Ok(_val) => {
                logger.log(&*format!("[RUST]: {} is already set.", key));
            }
            Err(_e) => {
                logger.log(&*format!("[RUST]: {} is not set, setting...", key));
                env::set_var(&key, &value);
            }
        }
    }
}


#[tauri::command]
pub fn check_env_var(name: &str, logger: State<FileLogger>) -> bool {
    logger.log(&*format!("[RUST]: Checking if {} is set", name));
    match env::var(name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
pub fn get_env_var(name: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => "".to_string(),
    }
}

#[tauri::command]
pub fn check_aws_auth_file(logger: State<FileLogger>) -> Result<bool, String> {
    return match home_dir() {
        None => {
            logger.log("[RUST]: Home directory not found");
            Err("[RUST]: Home directory not found".to_string())
        },
        Some(path) => {
            let auth_path = path.join(".aws").join("credentials");

            match auth_path.exists() {
                true => {
                    let mut file_map: HashMap<String, String> = HashMap::new();
                    let file = match File::open(auth_path) {
                        Ok(file) => file,
                        Err(_) => {
                            logger.log("[RUST]: AWS credentials file not found");
                            return Ok(false);
                        }
                    };

                    for line in BufReader::new(file).lines() {
                        let line = line.unwrap_or_default();
                        if line.is_empty() || line.contains("[default]") {
                            continue;
                        }
                        let mut split = line.splitn(2, '=');
                        let key = split.next().unwrap_or_default();
                        let value = split.next().unwrap_or_default();
                        file_map.insert(key.to_string(), value.to_string());
                    }

                    let expected_keys = vec!["aws_access_key_id", "aws_secret_access_key", "region"];

                    for key in expected_keys {
                        if !file_map.contains_key(key) {
                            logger.log("[RUST]: AWS credentials not found in file");
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                false => {
                    return Ok(false)
                }
            }
        }
    }
}

fn append_credentials(
    aws_access_key_id: &str,
    aws_secret_access_key: &str,
    region: &str,
    path: &PathBuf,
    logger: &FileLogger,
) -> Result<(), String> {
    logger.log("[RUST]: Adding creds to file");

    let path = path.join("credentials");

    let auth_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path).map_err(|e| {
        logger.log("[RUST]: An error occurred opening auth file");
        logger.log(&*e.to_string());
        e.to_string()
    })?;

    let mut writer = BufWriter::new(auth_file);

    writeln!(writer, "[default]").map_err(|e|{
        logger.log("[RUST]: An error occurred writing to auth file");
        logger.log(&*e.to_string());
        e.to_string()
    })?;

    writeln!(writer, "aws_access_key_id={}", aws_access_key_id).map_err(|e|{
        logger.log("[RUST]: An error occurred writing to auth file");
        logger.log(&*e.to_string());
        e.to_string()
    })?;

    writeln!(writer, "aws_secret_access_key={}", aws_secret_access_key).map_err(|e|{
        logger.log("[RUST]: An error occurred writing to auth file");
        logger.log(&*e.to_string());
        e.to_string()
    })?;

    writeln!(writer, "region={}", region).map_err(|e|{
        logger.log("[RUST]: An error occurred writing to auth file");
        logger.log(&*e.to_string());
        e.to_string()
    })?;

    Ok(())
}

fn set_aws_conf(
    aws_access_key_id: &str,
    aws_secret_access_key: &str,
    region: &str,
    path: &PathBuf,
    logger: &FileLogger
) -> Result<(), String> {
    logger.log("[RUST]: Checking aws conf file");
    let path = path.join(".aws");

    match path.exists() {
        true => {
            return Err("File exists. Use the update function instead".to_string())
        }
        false => {}
    }

    fs::create_dir(&path).map_err(|e| e.to_string())?;

    append_credentials(
        aws_access_key_id,
        aws_secret_access_key,
        region,
        &path,
        logger
    )
}

#[tauri::command]
pub fn write_aws_auth_to_file(
    aws_access_key_id: &str,
    aws_secret_access_key: &str,
    region: &str,
    logger: State<FileLogger>,
) -> Result<(), String> {
    return match home_dir() {
        None => {
            Err("Home directory not found".to_string())
        }
        Some(path) => {
            set_aws_conf(
                aws_access_key_id,
                aws_secret_access_key,
                region,
                &path,
                logger.inner()
            )
        }
    }
}

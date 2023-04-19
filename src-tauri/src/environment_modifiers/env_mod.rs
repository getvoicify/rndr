use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use tauri::State;
use tauri::api::path::home_dir;
use crate::utils::aws_credentials::{AwsCredentials, parse_credentials, write_credentials_to_file, write_hash_to_file};

use crate::utils::file_logger::FileLogger;
use crate::utils::logger::Logger;
use crate::utils::read_file_to_text_string::read_file_to_string;


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
pub fn get_aws_credentials(logger: State<FileLogger>) -> Result<AwsCredentials, String> {
    return match home_dir() {
        None => {
            logger.log("[RUST]: Home directory not found");
            Err("[RUST]: Home directory not found".to_string())
        }
        Some(path) => {
            let auth_path = path.join(".aws").join("credentials");
            return match auth_path.exists() {
                true => {
                    let file = match File::open(auth_path) {
                        Ok(file) => file,
                        Err(_) => {
                            logger.log("[RUST]: AWS credentials file not found");
                            return Err("[RUST]: AWS credentials file not found".to_string());
                        }
                    };
                    let mut aws_access_key_id: Option<String> = None;
                    let mut aws_secret_access_key: Option<String> = None;
                    let mut region: Option<String> = None;
                    for line in BufReader::new(file).lines() {
                        let line = line.unwrap_or_default();
                        if line.is_empty() || line.contains("[default]") {
                            continue;
                        }
                        let mut split = line.splitn(2, '=');
                        let key = split.next().unwrap_or_default().trim();
                        let value = split.next().unwrap_or_default().trim();
                        match key {
                            "aws_access_key_id" => aws_access_key_id = Some(value.to_string()),
                            "aws_secret_access_key" => aws_secret_access_key = Some(value.to_string()),
                            "region" => region = Some(value.to_string()),
                            _ => {}
                        }
                    }
                    Ok(AwsCredentials {
                        aws_access_key_id,
                        aws_secret_access_key,
                        region,
                    })
                }
                false => {
                    Err("[RUST]: ASW credentials not found".to_string())
                }
            }
        }
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
            let auth_file_path = auth_path.to_str().unwrap_or_default();

            let contents = parse_credential_from_file(logger.inner(), auth_file_path)?;

            let aws_credentials = parse_credentials(&contents);
            let mut credentials: Option<AwsCredentials> = None;
            for (key, value) in aws_credentials {
                if key == "rndr" {
                    credentials = Some(value);
                    break;
                }
            }

            return if let Some(_) = credentials {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

fn parse_credential_from_file(logger: &FileLogger, auth_file_path: &str) -> Result<String, String> {
    let contents = match read_file_to_string(auth_file_path) {
        Ok(contents) => contents,
        Err(_) => {
            logger.log("[RUST]: AWS credentials file not found");
            let file = File::create(auth_file_path).map_err(|e| e.to_string());
            match file {
                Ok(_) => {
                    logger.log("[RUST]: AWS credentials file created");
                    read_file_to_string(auth_file_path).map_err(|e| e.to_string())?
                }
                Err(e) => {
                    logger.log(&*format!("[RUST]: AWS credentials file not created - {}", e));
                    return Err(e);
                }
            }
        }
    };
    Ok(contents)
}

#[tauri::command]
pub fn write_aws_auth_to_file(aws_access_key_id: &str, aws_secret_access_key: &str, region: &str, logger: State<FileLogger>) -> Result<(), String> {
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

fn set_aws_conf(aws_access_key_id: &str, aws_secret_access_key: &str, region: &str, path: &PathBuf, logger: &FileLogger) -> Result<(), String> {
    logger.log("[RUST]: Checking aws conf file");
    let path = path.join(".aws");

    match path.exists() {
        true => {
            logger.log("[RUST]: AWS conf file exists");
        }
        false => {
            logger.log("[RUST]: AWS conf file does not exist");
            logger.log("[RUST]: Creating AWS conf file");
            fs::create_dir(&path).map_err(|e| e.to_string())?;
        }
    }


    append_credentials(
        aws_access_key_id,
        aws_secret_access_key,
        region,
        &path,
        logger
    )
}

fn append_credentials(aws_access_key_id: &str, aws_secret_access_key: &str, region: &str, path: &PathBuf, logger: &FileLogger) -> Result<(), String> {
    logger.log("[RUST]: Adding creds to file");

    let path = path.join("credentials");
    let auth_file_path = path.to_str().unwrap_or_default();

    logger.log(&*format!("[RUST]: Path to auth file - {}", auth_file_path));

    let contents = parse_credential_from_file(logger, auth_file_path)?;

    let aws_credentials = parse_credentials(&contents);
    let profile = "rndr";

    for (_, creds) in &aws_credentials {
        if creds.aws_access_key_id == Some(aws_access_key_id.to_string()) && creds.aws_secret_access_key == Some(aws_secret_access_key.to_string()) && creds.region == Some(region.to_string()) {
            logger.log("[RUST]: Credentials already exist");
            return Ok(());
        }
    }

    let updated_credentials = update_profile_credentials(
        aws_credentials,
        profile,
        Option::from(aws_access_key_id),
        Option::from(aws_secret_access_key),
        Option::from(region),
        logger
    ).unwrap_or_default();

    write_credentials_to_file(&updated_credentials, &path).map_err(|e| {
        logger.log("ERROR - [RUST]: An error occurred while writing credentials to file");
        e.to_string()
    })
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

fn update_profile_credentials(mut credentials: HashMap<String, AwsCredentials>, profile_name: &str, aws_access_key_id: Option<&str>, aws_secret_access_key: Option<&str>, region: Option<&str>, logger: &FileLogger) -> Result<HashMap<String, AwsCredentials>, String> {
    if let Some(profile) = credentials.get_mut(profile_name) {
        if let Some(aws_access_key_id) = aws_access_key_id {
            profile.aws_access_key_id = Some(aws_access_key_id.to_owned());
        }
        if let Some(aws_secret_access_key) = aws_secret_access_key {
            profile.aws_secret_access_key = Some(aws_secret_access_key.to_owned());
        }
        if let Some(region) = region {
            profile.region = Some(region.to_owned());
        }
        logger.log(&*format!("[RUST]: Profile {} updated", profile_name));
    } else {
        logger.log(&*format!("[RUST]: Profile {} not found", profile_name));
        credentials.insert(profile_name.to_owned(), AwsCredentials {
            aws_access_key_id: aws_access_key_id.map(|s| s.to_owned()),
            aws_secret_access_key: aws_secret_access_key.map(|s| s.to_owned()),
            region: region.map(|s| s.to_owned()),
        });
        logger.log(&*format!("[RUST]: Profile {} added", profile_name));
    }
    Ok(credentials)
}

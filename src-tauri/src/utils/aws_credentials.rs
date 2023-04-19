use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AwsCredentials {
    #[serde(rename = "awsAccessKeyId")]
    pub aws_access_key_id: Option<String>,
    #[serde(rename = "awsSecretAccessKey")]
    pub aws_secret_access_key: Option<String>,
    pub region: Option<String>,
}

pub fn parse_credentials(contents: &str) -> HashMap<String, AwsCredentials> {
    let mut aws_credentials: HashMap<String, AwsCredentials> = HashMap::new();
    let mut current_profile: Option<String> = None;

    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            // This line contains a profile name
            current_profile = Some(line[1..line.len() - 1].to_string());
        } else if let Some(profile) = &current_profile {
            // This line contains a credential for the current profile
            if let Some(index) = line.find('=') {
                let key = line[..index].trim();
                let value = line[index + 1..].trim();
                match key {
                    "aws_access_key_id" => {
                        aws_credentials
                            .entry(profile.clone())
                            .or_insert_with(|| AwsCredentials {
                                aws_access_key_id: Some(String::new()),
                                aws_secret_access_key: Some(String::new()),
                                region: None,
                            })
                            .aws_access_key_id = Some(value.to_string());
                    }
                    "aws_secret_access_key" => {
                        aws_credentials
                            .entry(profile.clone())
                            .or_insert_with(|| AwsCredentials {
                                aws_access_key_id: Some(String::new()),
                                aws_secret_access_key: Some(String::new()),
                                region: None,
                            })
                            .aws_secret_access_key = Some(value.to_string());
                    },
                    "region" => {
                        aws_credentials
                            .entry(profile.clone())
                            .or_insert_with(|| AwsCredentials {
                                aws_access_key_id: Some(String::new()),
                                aws_secret_access_key: Some(String::new()),
                                region: None,
                            })
                            .region = Some(value.to_string());
                    },
                    _ => (),
                }
            }
        }
    }

    aws_credentials
}

pub fn write_hash_to_file(filename: &str, file_map: HashMap<String, String>) {
    let mut file = BufWriter::new(
        File::create(filename).expect("Failed to create file"),
    );

    for (key, value) in file_map {
        writeln!(&mut file, "{}={}", key, value).expect("Failed to write line");
    }
}

pub fn write_credentials_to_file(credentials: &HashMap<String, AwsCredentials>, path: &PathBuf) -> Result<(), io::Error> {
    let file_path = path.to_str().unwrap_or_default();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    for (profile_name, credentials) in credentials {
        file.write_all(format!("[{}]\n", profile_name).as_bytes())?;
        if let Some(aws_access_key_id) = &credentials.aws_access_key_id {
            file.write_all(format!("aws_access_key_id = {}\n", aws_access_key_id).as_bytes())?;
        }
        if let Some(aws_secret_access_key) = &credentials.aws_secret_access_key {
            file.write_all(format!("aws_secret_access_key = {}\n", aws_secret_access_key).as_bytes())?;
        }
        if let Some(region) = &credentials.region {
            file.write_all(format!("region = {}\n", region).as_bytes())?;
        }
        file.write_all(b"\n")?;
    }

    Ok(())
}
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use tauri::{App, Wry};

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

pub fn run_bootstrap(file_name: &str, app: &mut App<Wry>) {
    bootstrap_env(file_name);
    // array of paths to create
    let app_data_dir = app.path_resolver().app_data_dir().unwrap();
    let app_data_dir = app_data_dir.to_str().unwrap();
    match Path::new(app_data_dir).exists() {
        true => println!("{} exists", app_data_dir),
        false => {
            println!("{} does not exist, creating...", app_data_dir);
            std::fs::create_dir_all(app_data_dir).unwrap();
        }
    }
}

#[tauri::command]
pub fn bootstrap_env(file_name: &str) {
    println!("Bootstrapping environment variables to {}", file_name);
    let var_hashmap = parse_file(file_name);

    for (key, value) in var_hashmap {
        match env::var(&key) {
            Ok(_val) => {
                println!("{} is already set.", key);
            }
            Err(_e) => {
                println!("{} is not set, setting...", key);
                env::set_var(&key, &value);
            }
        }
    }
}


#[tauri::command]
pub fn check_env_var(name: &str) -> bool {
    println!("Checking if {} is set", name);
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
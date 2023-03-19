use std::{env, io};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::string::String;
use tauri::{Window, Wry};
use crate::installers::dependency::Dependency;

pub struct PythonInstaller {
    pub version: String,
    pub python_type: String
}

impl Dependency for PythonInstaller {
    fn check(&mut self) -> bool {
        let python_check_command = Command::new("python")
            .arg("--version")
            .output();
        let python3_check_command = Command::new("python3")
            .arg("--version")
            .output();

        let has_python = match python_check_command {
            Ok(output) => {
                self.python_type = "python".to_string();
                output.status.success()
            }
            Err(_) => {
                false
            }
        };

        let has_python3 = match python3_check_command {
            Ok(output) => {
                self.python_type = "python3".to_string();
                output.status.success()
            }
            Err(_) => {
                false
            }
        };
        println!("Python is installed: {}", has_python || has_python3);
        has_python || has_python3
    }

    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing Python").unwrap();
        match install_python() {
            Ok(_) => {
                println!("Python installed successfully");
                true
            }
            Err(err) => {
                println!("Failed to install Python: {}", err);
                false
            }
        }
    }
}

fn download_python(url: &str, filename: &str) -> Result<(), String> {
    print!("Downloading Python... ");
    io::stdout().flush().unwrap();
    let output = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&filename)
        .arg(&url)
        .output()
        .map_err(|e| format!("Failed to download Python: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to download Python: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("done");
    Ok(())
}

fn install_python_binary(filename: &str) -> Result<(), String> {
    let os = env::consts::OS;

    print!("Installing Python... ");
    io::stdout().flush().unwrap();

    let output = match os {
        "windows" => {
            Command::new(&filename)
                .arg("/quiet")
                .arg("/passive")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|e| format!("Failed to install Python: {}", e))?
        }
        "macos" => {
            Command::new("sudo")
                .arg("installer")
                .arg("-pkg")
                .arg(&filename)
                .arg("-target")
                .arg("/")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|e| format!("Failed to install Python: {}", e))?
        }
        "linux" => {
            let path = String::from(&format!("{}-{}", &filename[..filename.len() - 4], os));
            let python_dir = Path::new(&path);
            Command::new("./configure")
                .current_dir(&python_dir)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|e| format!("Failed to configure Python: {}", e))?;

            Command::new("make")
                .current_dir(&python_dir)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|e| format!("Failed to build Python: {}", e))?;

            Command::new("sudo")
                .arg("make")
                .arg("install")
                .current_dir(&python_dir)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|e| format!("Failed to install Python: {}", e))?
        }
        _ => return Err("Unsupported operating system".into()),
    };

    if !output.status.success() {
        return Err(format!(
            "Failed to install Python: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("done");
    Ok(())
}

fn install_python() -> Result<(), String> {
    let os = env::consts::OS;

    let (url, filename) = match os {
        "windows" => (
            "https://www.python.org/ftp/python/3.10.0/python-3.10.0-amd64.exe",
            "python-3.10.0-amd64.exe",
        ),
        "macos" => (
            "https://www.python.org/ftp/python/3.10.0/python-3.10.0-macos11.pkg",
            "python-3.10.0-macos11.pkg",
        ),
        "linux" => (
            "https://www.python.org/ftp/python/3.10.0/Python-3.10.0.tgz",
            "Python-3.10.0.tgz",
        ),
        _ => return Err("Unsupported operating system".into()),
    };

    download_python(url, filename)?;
    install_python_binary(filename)?;

    Ok(())
}
use std::collections::HashMap;
use std::process::Command;
use tauri::{Window, Wry};
use crate::installers::dependency::Dependency;

pub struct PipInstaller {
    pub name: String,
    pub version: String,
}

struct PythonPackage {
    name: String,
    version: String,
}

impl Dependency for PipInstaller {
    fn check(&mut self) -> bool {
        match Command::new("python")
            .arg("-m")
            .arg("pip")
            .arg("--version")
            .output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout);
                version.contains(&self.version)
            }
            Err(err) => {
                println!("Error: {}", err);
                sentry::capture_error(&err);
                false
            }
        }
    }

    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing pip").unwrap();
        let output = Command::new("python")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg(format!("{}=={}", self.name, self.version))
            .output();

        match output {
            Ok(output) => output.status.success(),
            Err(err) => {
                println!("Error: {}", err);
                sentry::capture_error(&err);
                false
            }
        }
    }
}

impl Dependency for PythonPackage {
    fn check(&mut self) -> bool {
        // Check if the Python package is installed by running the pip freeze command
        let output = Command::new("pip")
            .arg("freeze")
            .output();

        match output {
            Ok(output) => {
                let installed_packages = String::from_utf8_lossy(&output.stdout);
                installed_packages.contains(&self.name)
            }
            Err(err) => {
                println!("Error: {}", err);
                sentry::capture_error(&err);
                false
            }
        }

    }

    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing python").unwrap();
        // Install the Python package using the pip command
        let output = Command::new("pip")
            .arg("install")
            .arg(format!("{}=={}", self.name, self.version))
            .output();
        match output {
            Ok(output) => output.status.success(),
            Err(err) => {
                println!("Error: {}", err);
                sentry::capture_error(&err);
                false
            }
        }
    }
}

pub fn dependencies() -> HashMap<String, impl Dependency> {

    let colorama: PythonPackage = PythonPackage {
        name: "colorama".to_string(),
        version: "0.4.6".to_string(),
    };

    let boto3: PythonPackage = PythonPackage {
        name: "boto3".to_string(),
        version: "1.17.3".to_string(),
    };
    let mut map = HashMap::new();
    map.insert(String::from("colorama"), colorama);
    map.insert(String::from("boto3"), boto3);
    map
}


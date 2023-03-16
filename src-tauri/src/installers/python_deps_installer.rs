use std::collections::HashMap;
use std::process::Command;
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
        let output = Command::new("pip")
            .arg("--version")
            .output()
            .expect("Failed to execute command");
        let version = String::from_utf8_lossy(&output.stdout);
        version.contains(&self.version)
    }

    fn install(&mut self) -> bool {
        let output = Command::new("python")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg(format!("{}=={}", self.name, self.version))
            .output()
            .expect("Failed to execute command");

        output.status.success()
    }
}

impl Dependency for PythonPackage {
    fn check(&mut self) -> bool {
        // Check if the Python package is installed by running the pip freeze command
        let output = Command::new("pip")
            .arg("freeze")
            .output()
            .expect("Failed to execute command");

        let installed_packages = String::from_utf8_lossy(&output.stdout);
        installed_packages.contains(&self.name)
    }

    fn install(&mut self) -> bool {
        // Install the Python package using the pip command
        let output = Command::new("pip")
            .arg("install")
            .arg(format!("{}=={}", self.name, self.version))
            .output()
            .expect("Failed to execute command");

        output.status.success()
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

    let sicaulc: PythonPackage = PythonPackage {
        name: "sicaulc".to_string(),
        version: "1.0".to_string(),
    };
    let mut map = HashMap::new();
    map.insert(String::from("colorama"), colorama);
    map.insert(String::from("boto3"), boto3);
    map.insert(String::from("sicaulc"), sicaulc);
    map
}


use std::process::{Command};
use tauri::{Window, Wry};

use crate::installers::dependency::Dependency;

pub struct OsDependencyInstaller {
    pub name: String,
    pub install_command: Command,
    pub version_check_command: Command,
}

impl Dependency for OsDependencyInstaller {
    fn check(&mut self) -> bool {
        let output = self.version_check_command.output().expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return output.status.success();
    }

    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", format!("Installing {}", self.name)).unwrap();
        let output = self.install_command.output().expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return output.status.success();
    }
}


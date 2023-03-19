use std::process::{Command, Stdio};
use tauri::{Window, Wry};
use crate::installers::dependency::Dependency;

pub struct Git;

impl Dependency for Git {
    fn check(&mut self) -> bool {
        Command::new("git")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_or(false, |status| status.success())
    }

    #[cfg(target_os = "linux")]
    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing git").unwrap();
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(r#"command -v git >/dev/null 2>&1 || { sudo apt-get update && sudo apt-get install -y git; }"#);

        let status = cmd.status().expect("failed to execute process");
        status.success()
    }

    #[cfg(target_os = "windows")]
    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing git").unwrap();
        // TODO: implement windows installer
        true
    }

    #[cfg(target_os = "macos")]
    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing git").unwrap();
        let output = Command::new("sh")
            .arg("-c")
            .arg(r#"command -v git >/dev/null 2>&1 || { /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)" && brew install git; }"#)
            .output()
            .expect("failed to execute process");
        output.status.success()
    }
}

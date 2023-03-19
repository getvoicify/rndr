use std::process::{Command};
use tauri::{Window, Wry};
use crate::installers::dependency::Dependency;
pub struct Docker;

impl Dependency for Docker {
    fn check(&mut self) -> bool {
        let output = Command::new("docker")
            .arg("--version")
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

    #[cfg(target_os = "windows")]
    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing docker").unwrap();
        let mut cmd = Command::new("powershell.exe");
        cmd.arg("-Command")
            .arg(r#"if (-not (Get-Service -Name Docker)) { Install-Module -Name DockerMsftProvider -Repository PSGallery -Force; Install-Package -Name docker -ProviderName DockerMsftProvider; Start-Service Docker }"#);

        let output = smd.output();

        match output {
            Ok(output) => output.status.success(),
            Err(err) => {
                println!("Error: {}", err);
                sentry::capture_error(&err);
                false
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing docker").unwrap();
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(r#"command -v docker >/dev/null 2>&1 || { sudo apt-get update && sudo apt-get install -y apt-transport-https ca-certificates curl gnupg-agent software-properties-common; curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -; sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"; sudo apt-get update && sudo apt-get install -y docker-ce docker-ce-cli containerd.io; }"#);

        match output {
            Ok(output) => output.status.success(),
            Err(err) => {
                println!("Error: {}", err);
                sentry::capture_error(&err);
                false
            }
        }
    }

    #[cfg(target_os = "macos")]
    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing docker").unwrap();
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(r#"command -v docker >/dev/null 2>&1 || { curl -fsSL https://download.docker.com/mac/stable/Docker.dmg -o $HOME/Downloads/Docker.dmg; open $HOME/Downloads/Docker.dmg; sleep 30; hdiutil attach /Volumes/Docker/Docker.app/Contents/Resources/dockerd-credential-osxkeychain; sudo cp -R /Volumes/Docker/Docker.app /Applications; hdiutil detach /Volumes/Docker; open /Applications/Docker.app; }"#);

        match cmd.output() {
            Ok(output) => output.status.success(),
            Err(err) => {
                println!("Error: {}", err);
                sentry::capture_error(&err);
                false
            }
        }
    }
}

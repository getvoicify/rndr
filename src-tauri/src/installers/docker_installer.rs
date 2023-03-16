use std::process::{Command, Stdio};
use crate::installers::dependency::Dependency;
pub struct Docker;

impl Dependency for Docker {
    fn check(&mut self) -> bool {
        Command::new("docker")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_or(false, |status| status.success())
    }

    #[cfg(target_os = "windows")]
    fn install(&mut self) -> bool {
        let mut cmd = Command::new("powershell.exe");
        cmd.arg("-Command")
            .arg(r#"if (-not (Get-Service -Name Docker)) { Install-Module -Name DockerMsftProvider -Repository PSGallery -Force; Install-Package -Name docker -ProviderName DockerMsftProvider; Start-Service Docker }"#);

        let status = cmd.status().expect("failed to execute process");

        status.success()
    }

    #[cfg(target_os = "linux")]
    fn install(&mut self) -> bool {
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(r#"command -v docker >/dev/null 2>&1 || { sudo apt-get update && sudo apt-get install -y apt-transport-https ca-certificates curl gnupg-agent software-properties-common; curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -; sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"; sudo apt-get update && sudo apt-get install -y docker-ce docker-ce-cli containerd.io; }"#);

        let status = cmd.status().expect("failed to execute process");

        status.success()
    }

    #[cfg(target_os = "macos")]
    fn install(&mut self) -> bool {
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(r#"command -v docker >/dev/null 2>&1 || { curl -fsSL https://download.docker.com/mac/stable/Docker.dmg -o $HOME/Downloads/Docker.dmg; open $HOME/Downloads/Docker.dmg; sleep 30; hdiutil attach /Volumes/Docker/Docker.app/Contents/Resources/dockerd-credential-osxkeychain; sudo cp -R /Volumes/Docker/Docker.app /Applications; hdiutil detach /Volumes/Docker; open /Applications/Docker.app; }"#);

        let status = cmd.status().expect("failed to execute process");

        status.success()
    }
}

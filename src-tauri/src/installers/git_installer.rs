use std::process::{Command, Stdio};
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
    fn install(&mut self) -> bool {
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(r#"command -v git >/dev/null 2>&1 || { sudo apt-get update && sudo apt-get install -y git; }"#);

        let status = cmd.status().expect("failed to execute process");
        status.success()
    }

    #[cfg(target_os = "windows")]
    fn install(&mut self) -> bool {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg("if (!(Get-Command git -ErrorAction SilentlyContinue)) \
            { Invoke-WebRequest https://github.com/git-for-windows/git/releases/download/v2.33.1.windows.1/Git-2.33.1-64-bit.exe \
            -OutFile $env:TEMP\\Git-2.33.1-64-bit.exe; Start-Process -FilePath $env:TEMP\\Git-2.33.1-64-bit.exe -ArgumentList \
            '/SILENT /NORESTART /COMPONENTS=\"icons,ext\\"/" -Wait }"
            )
            .output()
            .expect("failed to execute process");
        output.status.success()
    }

    #[cfg(target_os = "macos")]
    fn install(&mut self) -> bool {
        let output = Command::new("sh")
            .arg("-c")
            .arg(r#"command -v git >/dev/null 2>&1 || { /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)" && brew install git; }"#)
            .output()
            .expect("failed to execute process");
        output.status.success()
    }
}

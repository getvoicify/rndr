use std::process::Command;
use crate::installers::dependency::Dependency;

struct AwsCli {}

impl Dependency for AwsCli {
    fn check(&mut self) -> bool {
        let output = Command::new("aws")
            .arg("--version")
            .output()
            .expect("Failed to execute command");

        output.status.success()
    }

    fn install(&mut self) -> bool {
        // Install the AWS CLI depending on the operating system
        let output = if cfg!(target_os = "linux") {
            // Install on Linux using the curl command
            Command::new("curl")
                .arg("https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip")
                .output()
                .expect("Failed to execute command")
        } else if cfg!(target_os = "windows") {
            // Install on Windows using the PowerShell command
            Command::new("PowerShell")
                .arg("-Command")
                .arg("Invoke-WebRequest https://awscli.amazonaws.com/AWSCLIV2.msi -OutFile AWSCLIV2.msi; Start-Process msiexec -ArgumentList /i, AWSCLIV2.msi, /quiet -Wait")
                .output()
                .expect("Failed to execute command")
        } else if cfg!(target_os = "macos") {
            Command::new("sh")
                .arg("-c")
                .arg(r#"curl "https://awscli.amazonaws.com/AWSCLIV2.pkg" -o "AWSCLIV2.pkg" && sudo installer -pkg AWSCLIV2.pkg -target /"#)
                .output()
                .expect("Failed to execute command")
        } else {
            panic!("Unsupported operating system")
        };

        output.status.success()
    }
}
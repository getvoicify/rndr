use std::process::Command;
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::str;
use tauri::{Window, Wry};

fn pretty_print_error_message(message: &str) {
    println!("\x1B[31mError:\x1B[0m {}", message);
}

fn run_cmd(command: &mut Command) -> bool {
    match command.output() {
        Ok(output) => {
            if output.stdout.len() > 0 {
                println!("Log: {}", str::from_utf8(&output.stdout).unwrap());
            }
            if output.stderr.len() > 0 {
                pretty_print_error_message(str::from_utf8(&*output.stderr).unwrap());
            }
            output.status.success()
        },
        Err(e) => {
            println!("python failed: {:?}", e);
            return false;
        }
    }
}

#[tauri::command]
pub fn process_render(window: Window<Wry>, deps_path: &str, job_list: &str) {
    println!("job_list: {:?}", &job_list);
    let mut command = Command::new("python");
    command.arg(format!("{}/src/render.py", deps_path));
    command.arg("process");
    command.arg("-j");
    command.arg(job_list);
    spawn(move || {
        loop {
            let success = run_cmd(&mut command);
            if success {
                window.emit("update-process", success).unwrap();
            }
            sleep(Duration::from_secs(60));
        }
    });
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RenderConfig {
    scene: String,
    samples: u32,
    percentage: u32,
    startframe: u32,
    endframe: u32,
    breakpoint: Option<u32>,
}

#[tauri::command]
pub fn start_render(file_path: &str, config: RenderConfig, deps_path: &str, job_list: &str) {
    println!("start_render: {:?}", file_path);
    println!("start_render: {:?}", config);

    let mut command = Command::new("python");
    command.arg(format!("{}/src/render.py", deps_path));
    command.arg("add");
    command.arg("--blend");
    command.arg(&file_path);
    command.arg("--scene");
    command.arg(&config.scene);
    command.arg("--samples");
    command.arg(&config.samples.to_string());
    command.arg("--percentage");
    command.arg(&config.percentage.to_string());
    command.arg("--startframe");
    command.arg(&config.startframe.to_string());
    command.arg("--endframe");
    command.arg(&config.endframe.to_string());
    command.arg("-j");
    command.arg(&job_list);

    if config.breakpoint.is_some() {
        command.arg("--breaksize");
        command.arg(&config.breakpoint.unwrap().to_string());
    }

    run_cmd(&mut command);
}


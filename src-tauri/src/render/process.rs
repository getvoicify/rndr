use std::process::Command;
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::str;
use tauri::{Window, Wry};

fn pretty_print_error_message(message: &str) {
    println!("\x1B[31mError:\x1B[0m {}", message);
}

fn emit_message(window: &Window<Wry>, event: &str, payload: &str) {
    window.emit(event, payload).unwrap();
}

fn run_cmd<F, M>(command: &mut Command, err_cb: Option<F>, message_cb: Option<M>) -> bool where F: FnOnce(String), M: FnOnce(String) {
    match command.output() {
        Ok(output) => {
            if output.stdout.len() > 0 {
                println!("Log: {}", str::from_utf8(&output.stdout).unwrap());
                message_cb.map(|cb| cb(str::from_utf8(&output.stdout).unwrap().to_string()));
            }
            if output.stderr.len() > 0 {
                pretty_print_error_message(str::from_utf8(&*output.stderr).unwrap());
                err_cb.map(|f| f(str::from_utf8(&*output.stderr).unwrap().to_string()));
            }
            output.status.success()
        }
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
            let error_callback = |message: String| {
                emit_message(&window, "update-process", &message);
            };

            let message_callback = |message: String| {
                emit_message(&window, "update-process", &message);
            };
            let success = run_cmd(&mut command, Option::from(error_callback), Option::from(message_callback));
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

    run_cmd(&mut command, Some(|err: String| {
        pretty_print_error_message(&err);
    }), Some(|message: String| {
        pretty_print_error_message(&message);
    }));
}


use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;
use std::str;
use std::thread::spawn;

use tauri::{Window, Wry};

#[derive(Clone, serde::Serialize)]
enum StackEvent {
    Pending,
    Done,
    Error(String),
    Processing,
}

#[derive(Clone, serde::Serialize)]
struct Stack {
    stack_name: String,
    stack_status: StackEvent,
}


fn check_file_exists(filename: &str) -> bool {
    println!("Checking file: {}", filename);
    Path::new(filename).exists()
}

fn read_text_from_file(file_name: &str) -> String {
    let f = File::open(file_name).expect("Unable to open the file");
    let reader = BufReader::new(f);
    let mut text = String::new();
    for line in reader.lines() {
        let l = line.expect("Unable to read line");
        text.push_str(&l.trim_end_matches(|c| c == '\n' || c == '\r').to_string());
    }
    text
}

fn run_cmd(command: &mut Command) -> bool {
    match command.output() {
        Ok(output) => {
            println!("{:?}", str::from_utf8(&*output.stdout));
            println!("error: {:?}", str::from_utf8(&*output.stderr));
            output.status.success()
        },
        Err(e) => {
            println!("python failed: {:?}", e);
            return false;
        }
    }
}


fn create_stack_file(deps_path: &str, stack_name: &str) -> Result<(), ()> {
    match check_file_exists(&deps_path) {
        true => {
            println!("Stack file already exists");
            println!("Deleting stack file");
            fs::remove_file(&deps_path).unwrap();
        },
        false => {
            println!("Stack file does not exist");
            println!("Creating stack file");
        }
    }
    let mut file = fs::File::create(&deps_path).unwrap();
    writeln!(&mut file, "{}", stack_name).unwrap();
    Ok(())
}

#[tauri::command]
pub fn create_stack(window: Window<Wry>, deps_path: &str, stack_name: &str, stack_file: &str) {
    window.emit("create-stack", Stack {
        stack_name: stack_name.to_string(),
        stack_status: StackEvent::Pending,
    }).unwrap();
    let stack_name = match create_stack_file(&format!("{}.config/stack_name.txt", &deps_path), &stack_name) {
        Ok(_) => {
            read_text_from_file(&format!("{}.config/stack_name.txt", &deps_path))
        },
        Err(e) => {
            println!("Error creating stack file: {:?}", e);
            return ();
        }
    };

    window.emit("create-stack", Stack {
        stack_name: stack_name.to_string(),
        stack_status: StackEvent::Processing,
    }).unwrap();

    let mut aws_formation_command = Command::new("aws");
    aws_formation_command.arg("cloudformation");
    aws_formation_command.arg("deploy");
    aws_formation_command.arg("--template-file");
    aws_formation_command.arg(&stack_file);
    aws_formation_command.arg("--stack-name");
    aws_formation_command.arg(&stack_name);
    aws_formation_command.arg("--capabilities");
    aws_formation_command.arg("CAPABILITY_IAM");

    let mut command = Command::new("python");
    command.arg(format!("{}.brh-ext-deps/rendercli/src/render.py", deps_path));
    command.arg("configure");
    command.arg("--stackname");
    command.arg(&stack_name);

    spawn(move || {
        let is_stack_created = run_cmd(&mut aws_formation_command);
        match is_stack_created {
            true => match run_cmd(&mut command) {
                true => {
                    window.emit("create-stack", Stack { stack_name: stack_name.to_string(), stack_status: StackEvent::Done }).unwrap();
                },
                false => {
                    window.emit("create-stack", Stack { stack_name: stack_name.to_string(), stack_status: StackEvent::Error("Could not create stack file".to_string()) }).unwrap();
                }
            },
            false => {
                window.emit("create-stack", Stack { stack_name: stack_name.to_string(), stack_status: StackEvent::Error("Could not create stack".to_string()) }).unwrap();
            }
        }
    });
}

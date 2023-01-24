use std::process::Command;
use std::env;
use std::fs::File;
use std::path::Path;
use tauri::{App, Wry};

fn check_file_exists(file: &str) -> bool {
    let path = Path::new(file);
    path.exists()
}

#[tauri::command]
pub fn get_os() {
    let os = env::consts::OS;
    println!("OS: {}", os);
    match os {
        "windows" => Command::new("cmd")
            .args(&["/C", "ver"])
            .output()
            .expect("failed to execute process"),
        "macos" => Command::new("sw_vers")
            .output()
            .expect("failed to execute process"),
        "linux" => Command::new("uname")
            .arg("-a")
            .output()
            .expect("failed to execute process"),
        _ => Command::new("uname")
            .arg("-a")
            .output()
            .expect("failed to execute process"),
    };
}

#[tauri::command]
pub fn check_os_feature(feature: &str) -> bool {
    let output = match Command::new(feature).arg("--version").output() {
        Ok(output) => output,
        Err(_) => return false,
    };
    println!("output: {:?}", output);
    match output.status.success() {
        true => true,
        false => false,
    }
}

fn install_pip_deps(dep: &str) -> bool {
    match Command::new("pip")
        .arg("install")
        .arg(dep).output() {
        Ok(output) => match output.status.success() {
            true => {
                println!("pip install {:?} success", dep);
                true
            },
            false => {
                println!("pip install failed");
                false
            },
        },
        Err(e) => {
            println!("pip install colorama failed");
            println!("error: {:?}", e);
            false
        },
    }
}

#[tauri::command]
pub fn init_job_list(app: &mut App<Wry>) {
    let path = app.path_resolver().app_data_dir().unwrap();
    let path = path.join(".config").join(".joblist.csv");
    let path = path.to_str().unwrap();

    let deps_path = app.path_resolver().app_data_dir().unwrap();
    let deps_path = deps_path.join(".brh-ext-deps").join("rendercli");

    if !check_file_exists(deps_path.to_str().unwrap()) {
        println!("rendercli not found");
        return;
    }

    println!("job_list: {:?}", path);
    if !check_file_exists(&path) {
        println!("job_list does not exist");
        install_pip_deps("pip");
        let install_colorama: bool = install_pip_deps("colorama");
        let install_boto3: bool = install_pip_deps("boto3");

        if !install_colorama || !install_boto3 {
            println!("pip install failed");
            return;
        }

        println!("pip install success");
        println!("creating job_list");

        match Command::new("python")
            .arg(format!("{}/src/render.py", deps_path.to_str().unwrap()))
            .arg("init")
            .arg("-j")
            .arg(&path)
            .output() {
            Ok(output) => println!("output: {:?}", output),
            Err(error) => println!("error: {:?}", error),
        };
    }
}

#[tauri::command]
pub fn has_stack(home_path: &str) -> bool {
    check_file_exists(&format!("{}.renderconfig", home_path))
}



#[tauri::command]
pub fn create_blender_file(file_path: &str) -> Result<(), ()> {
    println!("create_blender_file: {:?}", file_path);
    match check_file_exists(file_path) {
        true => {
            // delete file
            match Command::new("rm")
                .arg(file_path)
                .output() {
                Ok(output) => println!("output: {:?}", output),
                Err(error) => println!("error: {:?}", error),
            };
        },
        false => {
            println!("file does not exist");
        },
    }
    match File::create(file_path) {
        Ok(_) => Ok(()),
        Err(error) => {
            println!("error: {:?}", error);
            Err(())
        },
    }
}

#[tauri::command]
pub fn create_blender_folder(home_path: &str) -> bool {
    let path = format!("{}/.config/.blender", home_path);
    if !check_file_exists(&path) {
        match Command::new("mkdir")
            .arg(&path)
            .output() {
            Ok(output) => {
                println!("output: {:?}", output);
                true
            },
            Err(error) => {
                println!("error: {:?}", error);
                false
            },
        }
    } else {
        true
    }
}
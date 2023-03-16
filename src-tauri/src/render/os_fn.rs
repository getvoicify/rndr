use std::process::Command;
use std::{env, fs};
use std::fs::File;
use std::path::{Path, PathBuf};
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


fn create_folder(path: &str) -> bool {
    match Command::new("mkdir")
        .arg(path)
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
}

#[tauri::command]
pub fn create_blender_folder(path: &str) -> bool {
    if !check_file_exists(&path) {
        create_folder(&path)
    } else {
        true
    }
}

#[tauri::command]
pub fn open_url(url: &str) {
    println!("open_url: {:?}", url);
    let os_name = env::consts::OS;
    let mut cmd = match os_name {
        "windows" => {
            let mut command = Command::new("cmd");
            command.args(&["/C", "start", url]);
            command
        },
        "linux" => {
            let mut command = Command::new("sh");
            command.arg("-c").arg(format!("xdg-open {}", url));
            command
        },
        "macos" => {
            let mut command = Command::new("sh");
            command.arg("-c").arg(format!("open {}", url));
            command
        },
        _ => return println!("Error: Unable to determine operating system"),
    };

    match cmd.spawn() {
        Ok(_) => println!("Opened {} in the default browser", url),
        Err(e) => println!("Error opening {}: {}", url, e),
    }
}

#[tauri::command]
pub fn open_folder_beginning_with_string(home_path: &str, folder_name_prefix: &str) {
    let paths = match fs::read_dir(home_path) {
        Ok(paths) => paths,
        Err(error) => {
            println!("error: {:?}", error);
            return;
        },
    };

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() && path.file_name().unwrap().to_str().unwrap().starts_with(folder_name_prefix) {
            #[cfg(target_os = "windows")]
            {
                let mut explorer = "explorer".to_owned();
                explorer.push_str(&format!(" {}", path.display()));
                std::process::Command::new("cmd")
                    .arg("/C")
                    .arg(explorer)
                    .spawn()
                    .expect("failed to start explorer");
            }
            #[cfg(target_os = "macos")]
            {
                let path_buf = PathBuf::from(&path);
                std::process::Command::new("open")
                    .arg("-R")
                    .arg(path_buf)
                    .spawn()
                    .expect("failed to start finder");
            }
            #[cfg(target_os = "linux")]
            {
                let path_buf = PathBuf::from(&path);
                std::process::Command::new("xdg-open")
                    .arg(path_buf)
                    .spawn()
                    .expect("failed to start file explorer");
            }
            return;
        }
    }
    println!("No folder found that begins with the given prefix: {}", folder_name_prefix);
}

#[tauri::command]
pub fn clone_git_project(path: &str) {
    let repo = "https://github.com/petetaxi-test/AwsBatchBlender";
    let mut command = Command::new("git");
    command.arg("clone");
    command.arg(repo);
    command.arg(path);
    command.output().expect("failed to start git");
}
use std::thread;
use std::sync::mpsc::{channel, TryRecvError};
use tauri::{AppHandle, Window, Wry};
use crate::installers::dependency::Dependency;
use crate::installers::git_installer::Git;
use crate::installers::python_deps_installer::{dependencies, PipInstaller};
use crate::installers::python_installer::PythonInstaller;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InstallerCheckResponse {
    name: String,
    is_installed: bool,
}


pub struct Installer {
    pub dependency: Box<dyn Dependency>,
    pub name: String,
}

impl Installer {
    fn install(&mut self, window: Window<Wry>) -> Result<(), String> {
        match self.dependency.check() {
            true => Ok(()),
            false => match self.dependency.install(window) {
                true => Ok(()),
                false => Err("Failed to install dependency".to_string()),
            },
        }
    }
}

#[tauri::command]
pub async fn start_installation(window: Window<Wry>, app_handle: AppHandle<Wry>) -> bool {
    println!("Starting installation...");
    let app_dir = app_handle.path_resolver().app_config_dir().unwrap();

    println!("App dir: {:?}", app_dir);

    let mut installers: Vec<Installer> = Vec::new();

    for installer in os_dependencies() {
        installers.push(installer);
    }

    for installer in install_pip() {
        installers.push(installer);
    }

    for installer in install_python_dependencies() {
        installers.push(installer);
    }

    for installer in install_python_dependencies() {
        installers.push(installer);
    }

    match installer(installers, window) {
        Ok(_) => true,
        Err(err) => {
            sentry::capture_message(&format!("Error: {}", err), sentry::Level::Error);
            false
        }
    }
}

fn installer(dep: Vec<Installer>, window: Window<Wry>) -> Result<bool, String> {
    println!("Installing dependencies...");
    let (tx, rx) = channel::<Result<bool, String>>();

    thread::spawn(move || {
        let mut all_installed = true;
        for mut d in dep {
            if let Err(e) = d.install(window.clone()) {
                all_installed = false;
                tx.send(Err(e)).map_err(|err| err.to_string())?;
                break;
            }
        }
        tx.send(Ok(all_installed)).map_err::<String, _>(|err| err.to_string())?;
        Ok::<(), String>(())
    });

    println!("Installation is running in the background...");

    loop {
        match rx.try_recv() {
            Ok(Ok(res)) => return Ok(res),
            Ok(Err(e)) => return Err(e),
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                return Err("Channel disconnected".to_string())
            }
        }
    }
}

#[tauri::command]
pub fn has_dependencies(app_handle: AppHandle<Wry>) -> Vec<InstallerCheckResponse> {
    println!("Checking dependencies...");
    let app_dir = app_handle.path_resolver().app_config_dir().unwrap();
    let os_path = app_dir.as_path().as_os_str().to_str().unwrap();
    sentry::capture_message(os_path, sentry::Level::Info);

    println!("App dir: {:?}", app_dir);

    let mut installers: Vec<Installer> = Vec::new();

    for installer in os_dependencies() {
        installers.push(installer);
    }

    let mut python = PythonInstaller {
        version: "3.9.2".to_string(),
        python_type: "python".to_string()
    };

    if !python.check() {
        println!("Python is not installed");
        installers.push(Installer {
            dependency: Box::new(python),
            name: "python".to_string()
        });
    }

    for installer in install_pip() {
        installers.push(installer);
    }

    // length of installers
    println!("Installers: {}", installers.len());

    match installers.is_empty() {
        true => {
            println!("No dependencies to install");
            sentry::capture_message("No dependencies to install", sentry::Level::Info);
            vec![]
        },
        false => installers.iter().map(|i| InstallerCheckResponse {
            name: i.name.clone(),
            is_installed: false
        }).collect()
    }
}

fn install_python_dependencies() -> Vec<Installer> {
    let python_dependencies = dependencies();
    let mut installers: Vec<Installer> = Vec::new();
    for (name, mut dep) in python_dependencies {
        let is_installed = dep.check();
        println!("{} is installed: {}", name, is_installed);
        if !is_installed {
            sentry::capture_message(&format!("Installing {}", name), sentry::Level::Info);
            let installer = Installer {
                dependency: Box::new(dep),
                name: name.to_string()
            };
            installers.push(installer);
        }
    }

    installers
}

fn install_pip() -> Vec<Installer> {
    let mut installers: Vec<Installer> = Vec::new();
    let mut pip: PipInstaller = PipInstaller {
        name: "pip".to_string(),
        version: "23.0.1".to_string(),
    };

    if !pip.check() {
        println!("Installing pip");
        sentry::capture_message(&format!("Installing {}", "pip"), sentry::Level::Info);
        let installer = Installer {
            dependency: Box::new(pip),
            name: "pip".to_string()
        };
        installers.push(installer);
    }
    installers
}

fn os_dependencies() -> Vec<Installer> {
    let mut installers: Vec<Installer> = Vec::new();
    match python_installer_factory() {
        Ok(installer) => {
            installers.push(installer);
        }
        Err(_) => {
            println!("Python is already installed");
        }
    }
    let mut git_installer = Git{};

    if !git_installer.check() {
        println!("Installing Git");
        let installer = Installer {
            dependency: Box::new(git_installer),
            name: "git".to_string()
        };
        installers.push(installer);
    }
    installers
}

fn python_installer_factory() -> Result<Installer, ()> {
    let mut python = PythonInstaller {
        version: "3.9.2".to_string(),
        python_type: "python".to_string()
    };
    if !python.check() {
        println!("Installing Python");
        sentry::capture_message(&format!("Installing {}", "python"), sentry::Level::Info);
        let python_installer = Installer {
            dependency: Box::new(python),
            name: "python".to_string()
        };
        return Ok(python_installer);
    }
    Err(())
}
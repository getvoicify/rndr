use std::thread;
use std::sync::mpsc::{channel, TryRecvError};
use tauri::{State, Window, Wry};
use crate::installers::dependency::Dependency;
use crate::installers::git_installer::Git;
use crate::installers::python_deps_installer::PipInstaller;
use crate::installers::python_installer::PythonInstaller;
use crate::utils::file_logger::FileLogger;
use crate::utils::logger::Logger;

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
pub async fn start_installation(window: Window<Wry>) -> bool {
    let logger = FileLogger {
        file_path: "/tmp/brh.log".to_string(),
    };

    logger.log("[RUST]: Starting installation...");

    let mut installers: Vec<Installer> = Vec::new();

    for installer in os_dependencies() {
        installers.push(installer);
    }

    // for installer in install_pip() {
    //     installers.push(installer);
    // }
    //
    // for installer in install_python_dependencies(&logger) {
    //     installers.push(installer);
    // }

    match installer(installers, window) {
        Ok(_) => true,
        Err(err) => {
            logger.log(&format!("Error: {}", err));
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
pub fn has_dependencies(state: State<FileLogger>) -> Vec<InstallerCheckResponse> {
    state.log("[RUST]: Checking dependencies...");

    let mut installers: Vec<Installer> = Vec::new();

    for installer in os_dependencies() {
        installers.push(installer);
    }
    let mut python = PythonInstaller {
        version: "3.9.2".to_string(),
        python_type: "python".to_string()
    };

    if !python.check() {
        state.log("[RUST]: Python is not installed");
        installers.push(Installer {
            dependency: Box::new(python),
            name: "python".to_string()
        });
    }

    for installer in install_pip() {
        installers.push(installer);
    }

    match installers.is_empty() {
        true => {
            state.log("[RUST]: No dependencies to install");
            sentry::capture_message("No dependencies to install", sentry::Level::Info);
            vec![]
        },
        false => installers.iter().map(|i| InstallerCheckResponse {
            name: i.name.clone(),
            is_installed: false
        }).collect()
    }
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
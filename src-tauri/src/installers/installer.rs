use std::thread;
use std::sync::mpsc::{channel, TryRecvError};

use crate::installers::dependency::Dependency;
use crate::installers::docker_installer::Docker;
use crate::installers::git_installer::Git;
use crate::installers::python_deps_installer::{dependencies, PipInstaller};
use crate::installers::python_installer::PythonInstaller;


pub struct Installer {
    pub dependency: Box<dyn Dependency>,
}

impl Installer {
    fn install(&mut self) -> Result<(), String> {
        match self.dependency.check() {
            true => Ok(()),
            false => match self.dependency.install() {
                true => Ok(()),
                false => Err("Failed to install dependency".to_string()),
            },
        }
    }
}

#[tauri::command]
pub fn start_installation() -> bool {
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

    match installer(installers) {
        Ok(_) => true,
        Err(_) => false
    }
}

fn installer(dep: Vec<Installer>) -> Result<bool, String> {
    println!("Installing dependencies...");
    let (tx, rx) = channel::<Result<bool, String>>();

    thread::spawn(move || {
        let mut all_installed = true;
        for mut d in dep {
            if let Err(e) = d.install() {
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
pub fn has_dependencies() -> bool {
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

    // length of installers
    println!("Installers: {}", installers.len());

    installers.is_empty()
}



fn install_python_dependencies() -> Vec<Installer> {
    let python_dependencies = dependencies();
    let mut installers: Vec<Installer> = Vec::new();
    for (name, mut dep) in python_dependencies {
        let is_installed = dep.check();
        println!("{} is installed: {}", name, is_installed);
        if !is_installed {
            let installer = Installer {
                dependency: Box::new(dep),
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
        let installer = Installer {
            dependency: Box::new(pip),
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
            dependency: Box::new(git_installer)
        };
        installers.push(installer);
    }

    let mut docker_installer = Docker {};
    if !docker_installer.check() {
        println!("Installing Docker");
        let installer = Installer {
            dependency: Box::new(docker_installer)
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
        let python_installer = Installer {
            dependency: Box::new(python)
        };
        return Ok(python_installer);
    }
    Err(())
}
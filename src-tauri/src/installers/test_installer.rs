use tauri::{Window, Wry};
use crate::installers::dependency::Dependency;

pub struct TestInstaller;

impl Dependency for TestInstaller {
    fn check(&mut self) -> bool {
        false
    }

    fn install(&mut self, window: Window<Wry>) -> bool {
        window.emit("inbound://installing_dependency", "Installing Test").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(10));
        true
    }
}
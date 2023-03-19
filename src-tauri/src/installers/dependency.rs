use tauri::{Window, Wry};

pub trait Dependency: Send {
    fn check(&mut self) -> bool;
    fn install(&mut self, window: Window<Wry>) -> bool;
}
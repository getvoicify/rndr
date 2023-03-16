pub trait Dependency: Send {
    fn check(&mut self) -> bool;
    fn install(&mut self) -> bool;
}
use crate::installers::dependency::Dependency;

pub struct TestInstaller;

impl Dependency for TestInstaller {
    fn check(&mut self) -> bool {
        false
    }

    fn install(&mut self) -> bool {
        std::thread::sleep(std::time::Duration::from_secs(10));
        true
    }
}
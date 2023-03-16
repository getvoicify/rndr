use std::collections::HashMap;
use crate::installers::dependency::Dependency;

pub fn dependency_checker(deps: HashMap<String, impl Dependency>) -> bool {
    // Iterate over the dependencies and check if they are installed
    let mut all_deps_installed = true;
    for (name, mut dep) in deps {
        if !dep.check() {
            println!("{} is not installed", name);
            all_deps_installed = false;
            break;
        }
    }
    all_deps_installed
}
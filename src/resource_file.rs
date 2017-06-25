use std::path::PathBuf;
use std::env;

use resources;

pub fn resource_path() -> PathBuf {
    let mut exe_path = env::current_exe()
        .expect("Failed to find executable path");

    exe_path.pop();

    exe_path.join(resources::RESOURCE_DIR)
}

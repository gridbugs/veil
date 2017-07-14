use std::path::{PathBuf, Path};
use std::env;

pub const RES_DIR: &'static str = "res";

pub const TILE_SHEET_IMAGE: &'static str = "tiles.png";
pub const TILE_SHEET_SPEC: &'static str = "tiles.toml";

pub fn res_dir() -> PathBuf {
    let mut exe_path = env::current_exe()
        .expect("Failed to find executable path");

    exe_path.pop();

    exe_path.join(RES_DIR)
}

pub fn res_path<P: AsRef<Path>>(path: P) -> PathBuf {
    res_dir().join(path)
}



use std::path::{PathBuf, Path};
use std::env;

pub const RESOURCE_DIR: &'static str = "resources";
pub const BUILD_RESOURCE_DIR: &'static str = "resources";

pub const TILE_SHEET_IMAGE: &'static str = "tiles.png";
pub const TILE_SHEET_SPEC: &'static str = "tiles.toml";

pub const TILE_SHEET_SCALE: u32 = 2;
pub const TILE_WIDTH_PX: u32 = 16;
pub const TILE_HEIGHT_PX: u32 = 16;

pub const TILE_SCALED_WIDTH_PX: u32 = TILE_WIDTH_PX * TILE_SHEET_SCALE;
pub const TILE_SCALED_HEIGHT_PX: u32 = TILE_HEIGHT_PX * TILE_SHEET_SCALE;

pub fn build_resource_dir_path() -> PathBuf {
    PathBuf::from(BUILD_RESOURCE_DIR)
}

pub fn build_resource_path<P: AsRef<Path>>(path: P) -> PathBuf {
    build_resource_dir_path().join(path)
}

pub fn out_dir_path() -> PathBuf {
    PathBuf::from(&env::var("OUT_DIR").expect("OUT_DIR is not set"))
}

pub fn out_path<P: AsRef<Path>>(path: P) -> PathBuf {
    out_dir_path().join(path)
}

pub fn stage_resource_dir_path() -> PathBuf {
    out_dir_path().join(RESOURCE_DIR)
}

pub fn stage_resource_path<P: AsRef<Path>>(path: P) -> PathBuf {
    stage_resource_dir_path().join(path)
}

pub fn resource_dir_path() -> PathBuf {
    let mut exe_path = env::current_exe()
        .expect("Failed to find executable path");

    exe_path.pop();

    exe_path.join(RESOURCE_DIR)
}

pub fn resource_path<P: AsRef<Path>>(path: P) -> PathBuf {
    resource_dir_path().join(path)
}

use simple_file;
use sdl2_frontend::tile::*;

pub fn launch() {
    let tile_desc_str = simple_file::read_string("resources/tiles.toml")
        .expect("Failed to open tile description");
    let tile_resolver = TileResolver::from_str(&tile_desc_str);
    println!("{:?}", tile_resolver);
}

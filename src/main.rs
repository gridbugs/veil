#![allow(dead_code)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate enum_primitive;

extern crate rand;
extern crate sdl2;
extern crate toml;

mod grid;
#[macro_use] mod entity_store;
mod spatial_hash;

mod content;
mod sdl2_frontend;
mod simple_file;
mod entity_id_allocator;
mod knowledge;

mod tests;

fn main() {
    sdl2_frontend::launch();
}

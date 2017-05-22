#![allow(dead_code)]
#![allow(unused_macros)]
#![feature(inclusive_range_syntax)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate itertools;

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
mod observation;
mod direction;
mod policy;
mod straight_line;
mod vector_index;
mod grid_search;
mod best;
mod invert_ord;
mod behaviour;
mod coord;

mod tests;

fn main() {
    sdl2_frontend::launch();
}

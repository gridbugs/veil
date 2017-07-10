#![allow(dead_code)]
#![allow(unused_macros)]
#![feature(float_bits_conv)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate itertools;
#[macro_use] extern crate maplit;

extern crate rand;
extern crate toml;
extern crate handlebars;

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate gfx_text;
extern crate winit;
extern crate genmesh;
extern crate image;

mod grid;
#[macro_use] mod entity_store;
mod spatial_hash;

mod glutin_frontend;

mod content;
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
mod render_overlay;
mod limits;
mod schedule;
mod frame;
mod reaction;
mod entity_observe;
mod meta_action;
mod renderer;
mod input;
mod turn;
mod player_act;
mod npc_act;
mod player_render;
mod commit;
mod perlin;
mod veil_state;
mod terrain;
mod tile;
mod tile_desc;
mod tile_buffer;
mod level_env;
mod game_env;
mod launch;
mod resources;
mod common_input;

mod tests;

fn main() {
    glutin_frontend::launch();

}

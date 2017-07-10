#![feature(float_bits_conv)]
#![allow(dead_code)]

#[macro_use] extern crate maplit;
extern crate toml;
extern crate handlebars;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate gfx_text;
extern crate winit;
extern crate genmesh;
extern crate image;

extern crate util;
extern crate geometry;
extern crate game_policy;
extern crate game_data;

mod launcher;
mod frontend;
mod input;
mod tile_map;
mod formats;
mod sizes;
mod world_tile;
mod overlay_tile;
pub use self::launcher::*;

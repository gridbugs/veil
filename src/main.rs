#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![feature(float_bits_conv)]

extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate enum_primitive;
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

extern crate util;
extern crate geometry;
#[macro_use]
extern crate game_data;
extern crate game_policy;

mod glutin_frontend;

mod tests;

fn main() {
    glutin_frontend::launch();

}

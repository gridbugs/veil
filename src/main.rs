#![allow(unused_imports)]

extern crate cgmath;
extern crate rand;

extern crate util;
extern crate geometry;
#[macro_use]
extern crate game_data;
extern crate game_policy;
extern crate glutin_frontend;

mod tests;

fn main() {
    glutin_frontend::launch();
}

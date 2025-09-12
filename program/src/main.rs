use std::{fmt::Debug, fs::File, io::Read, path::Path};

use gltech3::prelude::*;

extern crate gltech3;
extern crate sdl2;
extern crate thiserror;

mod load_image;
mod scene_test;

pub fn main() {
    scene_test::main();
}

extern crate gltech;
extern crate sdl2;
extern crate thiserror;
extern crate zip;

mod file_system;
mod image;
mod scene_test;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    scene_test::main()?;
    Ok(())
}

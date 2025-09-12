extern crate gltech;
extern crate sdl2;
extern crate thiserror;
extern crate zip;

mod file_system;
mod image;
mod scene_test;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = scene_test::main();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        eprintln!("Caused by: {:?}", e);
        std::process::exit(1);
    }
    Ok(())
}

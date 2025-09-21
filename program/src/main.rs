use std::ffi::CStr;

extern crate gltech;
extern crate image;
extern crate thiserror;
extern crate zip;

mod file_system;
mod images;
mod map_pixel_benchmark;
mod test_scene;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = map_pixel_benchmark::main();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        eprintln!("Caused by: {:?}", e);
        std::process::exit(1);
    }
    Ok(())
}

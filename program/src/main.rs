extern crate gltech;
extern crate image;
extern crate thiserror;
extern crate zip;

mod file_system;
mod images;
mod test_scene;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = test_scene::main();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        eprintln!("Caused by: {:?}", e);
        std::process::exit(1);
    }
    Ok(())
}

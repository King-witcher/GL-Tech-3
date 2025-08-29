extern crate sdl2;
mod image;
mod window;

use sdl2::sys::*;
use window::Window;

pub fn main() -> Result<(), String> {
    let window = Window::new("GLTech 3", 800, 600, 800, 600, false);
    let image = window.image();
    let mut color = 0u32;

    'main_loop: loop {
        for event in Window::events() {
            match event {
                SDL_EventType::SDL_QUIT => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        for i in 0..800 {
            for j in 0..600 {
                image.set(i, j, color / 10);
            }
        }

        color += 5;
        window.update();
    }
    Ok(())
}

extern crate sdl2;

mod image;
mod sdl;
mod window;

use window::Window;

use crate::sdl::SDLEvent;

pub fn main() {
    let window = Window::new("GLTech 3", 800, 450, 800, 450, false);
    let image = window.image();
    let mut counter = 0u32;

    'main_loop: loop {
        for event in Window::events() {
            match event {
                SDLEvent::Quit(_) => {
                    break 'main_loop;
                }
                SDLEvent::KeyDown(e) => {
                    println!("{:?}", e.keysym.sym);
                }
                _ => {}
            }
        }

        for i in 0..800 {
            for j in 0..450 {
                let color = if ((counter / 1000) % 2) == 0u32 {
                    0xffffffffu32
                } else {
                    0
                };
                image.set(i, j, color);
            }
        }

        counter += 1;
        window.update();
    }
}

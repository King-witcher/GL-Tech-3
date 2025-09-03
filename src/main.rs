extern crate sdl2;

mod imaging;
mod sdl;
// mod threading;
mod threading;
mod window;
mod world;

use std::sync::Mutex;

use sdl2::{pixels::PixelFormatEnum, sys::SDL_Scancode};
use window::Window;

use crate::{
    sdl::{SDLEvent, SDLKeysym},
    threading::GLArc,
};

pub fn main() {
    tests();
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window("janela", 800, 450).build().unwrap();

    return;
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
                    if e.keysym.scancode == SDL_Scancode::SDL_SCANCODE_ESCAPE {
                        break 'main_loop;
                    }
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

struct MutateMe {
    pub inner: i32,
}

impl MutateMe {
    fn mutate(&mut self) {
        self.inner = 666;
    }
}

fn tests() {
    let mutate_me = MutateMe { inner: 333 };
    let mutate_me = GLArc::new(mutate_me);
    let clone = mutate_me.clone();
    drop(mutate_me);
    drop(clone);
}

fn create_window() {
    let context = sdl2::init().unwrap();
    context.video().unwrap().window("title", 0, 0);
}

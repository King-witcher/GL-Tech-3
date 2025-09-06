extern crate sdl2;

mod imaging;
mod renderer;
mod sdl;
mod threading;
mod vector;
mod window;
mod world;

use sdl2::sys::SDL_Scancode;
use window::Window;

use crate::sdl::SDLEvent;

pub fn main() {
    let mut scene = world::Scene::new();
    renderer::run(&mut scene);
}

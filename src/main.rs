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

use crate::{renderer::Renderer, sdl::SDLEvent, vector::Vector, world::Plane};

pub fn main() {
    let mut scene = world::Scene::new();
    let plane = Plane::new(Vector(1.0, 1.0), Vector(1.0, -1.0));
    scene.planes.push(plane);
    let renderer = Renderer::new(scene);
    renderer.start();
}

extern crate sdl2;

mod imaging;
mod renderer;
mod sdl;
mod threading;
mod vector;
mod window;
mod world;

use crate::{renderer::Renderer, vector::Vector, world::Plane};

pub fn main() {
    let mut scene = world::Scene::new();
    let plane = Plane::new(Vector(1.0, 1.0), Vector(1.0, -2.0));
    scene.planes.push(plane);
    let renderer = Renderer::new(scene);
    renderer.start();

    let vector = (1.0, 0.0);
    vector.module();
}

trait VectorTest {
    fn module(&self) -> f32;
}

impl VectorTest for (f32, f32) {
    fn module(&self) -> f32 {
        f32::sqrt(self.0 * self.0 + self.1 * self.1)
    }
}

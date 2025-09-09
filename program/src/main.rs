extern crate gltech3;
extern crate sdl2;

use gltech3::{renderer::Renderer, vector::Vector, world::Plane, *};

trait A {
    fn foo(&self);
}

impl dyn A {
    fn bar(&self) {
        self.foo();
    }
}

struct I;

impl A for I {
    fn foo(&self) {
        println!("foo");
    }
}

pub fn main() {
    let mut scene = world::Scene::new();
    let plane = Plane::new(Vector(1.0, 1.0), Vector(1.0, -2.0));
    scene.add_node(plane.into());
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

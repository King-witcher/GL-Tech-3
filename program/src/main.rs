extern crate gltech3;
extern crate sdl2;

use gltech3::{
    renderer::Renderer,
    scripting::{script::Script, UpdateContext},
    vector::Vector,
    world::{Entity, EntityNode, Plane},
    *,
};

pub fn main() {
    let mut scene = world::Scene::new();
    let plane = Plane::new(Vector(1.0, 1.0), Vector(1.0, -2.0));
    let mut plane: EntityNode = plane.into();
    plane.add_script(Box::new(RotateScript));
    scene.add_node(plane);
    let renderer = Renderer::new(scene);
    renderer.start();
}

pub struct RotateScript;

impl Script for RotateScript {
    fn start(&mut self, _ctx: &gltech3::scripting::script::StartContext) {}

    fn update(&mut self, ctx: &mut UpdateContext) {
        ctx.entity.r#move(Vector(0.01, 0.01));
    }

    fn end(&mut self, _ctx: &gltech3::scripting::script::EndContext) {}
}

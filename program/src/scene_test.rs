use std::path::Path;

use gltech3::{
    prelude::*,
    renderer::RendererBuilder,
    scripting::{Script, UpdateContext},
    world::{Entity, Plane},
    Image, Texture,
};

use crate::load_image::load_image;

pub fn main() {
    let mut scene = Scene::new();
    let path = Path::new("./assets/bmp.bmp");
    let image = load_image(path).unwrap();
    let image_ref = unsafe { &*(&image as *const Image) };
    let texture = Texture::new(image_ref);
    let plane = Plane::new(Vector(2.0, 0.0), Vector(1.0, 0.0), texture);
    let mut plane: Entity = plane.into();
    plane.add_script(Box::new(RotateScript));
    scene.add_node(plane);
    let renderer = RendererBuilder::new(scene)
        .width(1920)
        .height(1080)
        .fullscreen();
    renderer.start();
}

struct RotateScript;

impl Script for RotateScript {
    fn start(&mut self, _ctx: &gltech3::scripting::script::StartContext) {}

    fn update(&mut self, ctx: &mut UpdateContext) {
        ctx.entity.rotate(1.0);
        // print!("Module: {:?}\n", ctx.entity.dir());
    }

    fn end(&mut self, _ctx: &gltech3::scripting::script::EndContext) {}
}

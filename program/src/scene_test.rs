use gltech::{
    prelude::*,
    renderer::RendererBuilder,
    scripting::{Script, UpdateContext},
    world::{Entity, Plane},
    Image, Texture,
};

use crate::{file_system, image};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_system = file_system::load_file_system()?;
    let wood = file_system.get("wood.bmp")?;

    let mut scene = Scene::new();
    let image = image::get_image(wood).unwrap();
    let image_ref = unsafe { &*(&image as *const Image) };
    let texture = Texture::new(image_ref);
    let plane = Plane::new(Vector(1.0, 0.0), Vector(1.0, 0.0), texture);
    let mut plane: Entity = plane.into();
    plane.add_script(Box::new(RotateScript));
    scene.add_node(plane);
    let renderer = RendererBuilder::new(scene)
        .width(1920)
        .height(1080)
        .fullscreen();
    renderer.start();

    Ok(())
}

struct RotateScript;

impl Script for RotateScript {
    fn start(&mut self, _ctx: &gltech::scripting::script::StartContext) {}

    fn update(&mut self, ctx: &mut UpdateContext) {
        ctx.entity.rotate(1.0);
    }

    fn end(&mut self, _ctx: &gltech::scripting::script::EndContext) {}
}

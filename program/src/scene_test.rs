use gltech::{
    prelude::*,
    renderer::RendererBuilder,
    scripting::{Script, UpdateContext},
    world::{Entity, Plane},
    Texture,
};

use crate::{file_system, images};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_system = file_system::load_file_system()?;
    let glass4a = file_system.get("textures/glass/marineglass4a.dds")?;
    let image = images::get_from_file(glass4a)?;

    let mut scene = Scene::new();
    let texture: Texture = Texture::new(&image);
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

use std::rc::Rc;

use gltech::{
    engine,
    prelude::*,
    scripting::{Script, UpdateContext},
    world::{Entity, Plane},
    Texture,
};

use crate::{file_system, images};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_system = file_system::load_file_system()?;
    let bianca = file_system.get("bianca.jpg")?;
    let image = images::get_from_file(bianca)?;
    let image = Rc::new(image);
    let mut scene = Scene::new();

    // Rotating plane 1
    {
        let texture = Texture::new(image.clone());
        let primitive = Plane::new(Vector(1.0, -0.25), Vector(1.0, 0.0), texture);
        let mut entity = Entity::from_plane(primitive);
        entity.add_script(Box::new(RotateScript));
        scene.add_node(entity);
    }
    // Rotating plane 2
    {
        let texture = Texture::new(image.clone());
        let primitive = Plane::new(Vector(1.0, 0.25), Vector(0.0, 1.0), texture);
        let mut entity = Entity::from_plane(primitive);
        entity.add_script(Box::new(RotateScript));
        scene.add_node(entity);
    }

    let mut engine = engine::init()?;
    engine.fullscreen(false).title("GLTech 3").vsync(false);

    engine.launch(scene)?;

    Ok(())
}

struct RotateScript;

impl Script for RotateScript {
    fn start(&mut self, _ctx: &gltech::scripting::script::StartContext) {}

    fn update(&mut self, ctx: &mut UpdateContext) {
        let delta_time = ctx.delta_time.as_secs_f32();
        ctx.entity.rotate(90.0 * delta_time);
    }

    fn end(&mut self, _ctx: &gltech::scripting::script::EndContext) {}
}

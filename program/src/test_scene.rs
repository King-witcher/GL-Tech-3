mod rotate_script;

use crate::{file_system, images, test_scene::rotate_script::RotateScript};
use gltech::{
    engine,
    prelude::*,
    standard::FlatPlayerController,
    world::{Entity, Plane},
    Texture,
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_system = file_system::load_file_system()?;
    let bianca = file_system.get("bianca.jpg")?;
    let image = images::get_from_file(bianca)?;
    let mut scene = Scene::new();

    // Rotating plane 1
    {
        let texture = Texture::new(image.cheap_clone());
        let primitive = Plane::new(Vector(1.0, -0.25), Vector(1.0, 0.0), texture);
        let mut entity = Entity::from(primitive);
        entity.add_script(Box::new(RotateScript));
        scene.add(entity);
    }
    // Rotating plane 2
    {
        let texture = Texture::new(image);
        let primitive = Plane::new(Vector(1.0, 0.25), Vector(0.0, 1.0), texture);
        let mut entity = Entity::from(primitive);
        entity.add_script(Box::new(RotateScript));
        entity.add_script(Box::new(FlatPlayerController::default()));
        scene.add(entity);
    }

    let mut engine = engine::init()?;

    engine.fullscreen(false).title("GLTech 3").vsync(false);
    engine.launch(scene)?;

    Ok(())
}

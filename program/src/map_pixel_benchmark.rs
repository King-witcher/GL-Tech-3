use std::sync::Arc;

use gltech::scripting::UpdateContext;
use gltech::{prelude::*, Entity, Plane, Scene, Script, Texture};

use crate::file_system::load_file_system;

// This commit: 67 fps (map nearest)
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_system = load_file_system()?;
    let bianca = file_system.get("bmp.bmp")?;
    let image = crate::images::get_from_file(bianca)?;
    let image = Arc::new(image);
    let mut scene = Scene::new();

    // Plane
    {
        let texture = Texture::new(image);
        let plane = Plane::new(Vector(0.5, 1.0), Vector(0.0, -2.0), texture);
        let mut entity = Entity::from_plane(plane);
        let script = BenchmarkScript::new();
        entity.add_script(Box::new(script));
        scene.add_node(entity);
    }

    let mut engine = engine::init()?;
    engine
        .title("Map Pixel Benchmark")
        .resolution(1600, 900)
        // .fullscreen(true)
        .vsync(false);
    engine.launch(scene)?;

    Ok(())
}

struct BenchmarkScript {
    frames: f32,
    delta: f32,
}

impl BenchmarkScript {
    fn new() -> Self {
        Self {
            frames: 0.0,
            delta: 0.0,
        }
    }
}

impl Script for BenchmarkScript {
    fn start(&mut self, _ctx: &gltech::scripting::script::StartContext) {}

    fn update(&mut self, ctx: &mut UpdateContext) {
        self.frames += 1.0;
        self.delta += ctx.delta_time.as_secs_f32();
        if self.delta >= 5.0 {
            let fps = self.frames / self.delta;
            println!("FPS: {}", fps);
            self.delta = 0.0;
            self.frames = 0.0;
        }
    }

    fn end(&mut self, _ctx: &gltech::scripting::script::EndContext) {}
}

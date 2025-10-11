use sdl2::keyboard::Scancode;

use crate::{EndContext, Script, Spatial, StartContext, UpdateContext};

pub struct MovePlayer;

impl Script for MovePlayer {
    fn start(&mut self, ctx: StartContext) {
        ctx.system.set_capture_mouse(true);
    }

    fn tick(&mut self, ctx: UpdateContext) {
        let delta_time = ctx.delta_time.as_secs_f32();
        if ctx.input.is_key_down(Scancode::W) {
            ctx.scene.camera.ray.start += delta_time * ctx.scene.camera.ray.dir;
        }
        if ctx.input.is_key_down(Scancode::S) {
            ctx.scene.camera.ray.start -= delta_time * ctx.scene.camera.ray.dir;
        }

        let mouse_delta = ctx.input.mouse_rel().0;
        println!("Mouse delta: {}", mouse_delta);
        ctx.scene
            .camera
            .ray
            .rotate(-mouse_delta as f32 * 2.2 * 0.022);
    }

    fn end(&mut self, _ctx: EndContext) {}
}

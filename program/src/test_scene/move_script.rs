use gltech::prelude::*;
use gltech::EndContext;
use gltech::Scancode;
use gltech::Script;
use gltech::StartContext;
use gltech::UpdateContext;

pub struct MovePlayer;

impl Script for MovePlayer {
    fn start(&mut self, ctx: StartContext) {
        ctx.system.set_capture_mouse(false);
    }

    fn tick(&mut self, ctx: UpdateContext) {
        let delta_time = time::delta_time().as_secs_f32();
        if ctx.input.is_key_down(Scancode::W) {
            ctx.scene.camera.ray.start += delta_time * ctx.scene.camera.ray.dir;
        }
        if ctx.input.is_key_down(Scancode::S) {
            ctx.scene.camera.ray.start -= delta_time * ctx.scene.camera.ray.dir;
        }
    }

    fn end(&mut self, _ctx: EndContext) {}
}

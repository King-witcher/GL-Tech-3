use gltech::Scancode;
use gltech::Script;

pub struct MovePlayer;

impl Script for MovePlayer {
    fn start(&mut self, _ctx: &gltech::scripting::script::StartContext) {}

    fn update(&mut self, ctx: gltech::UpdateContext) {
        let delta_time = gltech::engine::time::delta_time().as_secs_f32();
        if ctx.input.is_key_down(Scancode::W) {
            ctx.scene.camera.ray.start += delta_time * ctx.scene.camera.ray.dir;
        }
        if ctx.input.is_key_down(Scancode::S) {
            ctx.scene.camera.ray.start -= delta_time * ctx.scene.camera.ray.dir;
        }
    }

    fn end(&mut self, _ctx: &gltech::scripting::script::EndContext) {}
}

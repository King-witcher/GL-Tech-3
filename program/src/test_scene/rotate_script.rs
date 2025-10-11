use gltech::{engine::time, EndContext, Script, StartContext, UpdateContext};

pub struct RotateScript;

impl Script for RotateScript {
    fn start(&mut self, _ctx: StartContext) {}

    fn tick(&mut self, ctx: UpdateContext) {
        let delta_time = time::delta_time().as_secs_f32();
        ctx.entity.rotate(90.0 * delta_time);
    }

    fn end(&mut self, _ctx: EndContext) {}
}

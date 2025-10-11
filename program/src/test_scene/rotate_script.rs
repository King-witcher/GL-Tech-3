use gltech::{EndContext, Script, StartContext, UpdateContext};

pub struct RotateScript;

impl Script for RotateScript {
    fn start(&mut self, _ctx: StartContext) {}

    fn tick(&mut self, ctx: UpdateContext) {
        let delta_time = ctx.delta_time.as_secs_f32();
        ctx.entity.rotate(9.0 * delta_time);
    }

    fn end(&mut self, _ctx: EndContext) {}
}

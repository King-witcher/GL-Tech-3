use sdl2::keyboard::Scancode;

use crate::{EndContext, Input, Script, StartContext, UpdateContext, Vector};

pub struct FlatPlayerController {
    pub speed: f32,
    pub vertical_speed: f32,
    pub m_sensitivity: f32,
}

impl Default for FlatPlayerController {
    fn default() -> Self {
        Self {
            speed: 100.0,
            vertical_speed: 100.0,
            m_sensitivity: 2.2,
        }
    }
}

impl FlatPlayerController {
    fn wish_dir(look_dir: Vector, input: Input) -> Vector {
        let mut dir = Vector::ZERO;

        if input.is_key_down(Scancode::W) {
            dir += Vector::FORWARD;
        }
        if input.is_key_down(Scancode::S) {
            dir += Vector::BACK;
        }
        if input.is_key_down(Scancode::A) {
            dir += Vector::LEFT;
        }
        if input.is_key_down(Scancode::D) {
            dir += Vector::RIGHT;
        }

        if dir.mag() != 0.0 {
            dir.modularize();
        }

        dir.cmul(look_dir)
    }
}

impl Script for FlatPlayerController {
    fn start(&mut self, ctx: StartContext) {
        ctx.system.set_capture_mouse(true);
    }

    fn tick(&mut self, ctx: UpdateContext) {
        let delta_time = ctx.delta_time.as_secs_f32();

        let wish_dir = Self::wish_dir(ctx.scene.camera.ray.dir, ctx.input.clone());
        ctx.scene
            .camera
            .translate(wish_dir * self.speed * delta_time);

        let mouse_delta = ctx.input.mouse_rel().0;
        ctx.scene
            .camera
            .rotate(self.m_sensitivity * -0.022 * mouse_delta as f32);

        if ctx.input.is_key_down(Scancode::Space) {
            ctx.scene.camera.z =
                f32::min(ctx.scene.camera.z + self.vertical_speed * delta_time, 1.0);
        }

        if ctx.input.is_key_down(Scancode::LAlt) {
            ctx.scene.camera.z =
                f32::max(ctx.scene.camera.z - self.vertical_speed * delta_time, 0.0);
        }
    }

    fn end(&mut self, _ctx: EndContext) {}
}

use sdl2::keyboard::Scancode;

use crate::EndContext;
use crate::Input;
use crate::Script;
use crate::StartContext;
use crate::UpdateContext;
use crate::prelude::*;

pub struct Q1Controller {
    pub acceleration: f32,
    pub air_acceleration: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub sensitivity: f32,
    pub friction: f32,
    pub height: f32,
    pub max_speed: f32,
    pub stop_speed: f32,

    velocity: Vector,
    z_speed: f32,
    grounded: bool,
}

impl Default for Q1Controller {
    fn default() -> Self {
        Self {
            acceleration: 7.0,
            air_acceleration: 10.0,
            jump_speed: 2.7,
            gravity: 8.0,
            sensitivity: 2.2,
            friction: 6.0,
            height: 0.45,
            max_speed: 3.2,
            stop_speed: 3.0,

            velocity: Vector::ZERO,
            grounded: true,
            z_speed: 0.0,
        }
    }
}

impl Q1Controller {
    fn update_velocity(&mut self, ctx: &UpdateContext) {
        let wishdir = Self::wishdir(ctx.scene.camera.ray.dir, ctx.input.clone());

        if self.grounded {
            self.accelerate(ctx, wishdir, self.max_speed);
        }
    }

    fn update_z(&mut self, ctx: &mut UpdateContext, delta_time: f32) {
        if self.grounded {
            return;
        }

        ctx.scene.camera.z += self.z_speed * delta_time;
        if ctx.scene.camera.z < self.height {
            ctx.scene.camera.z = self.height;
            self.grounded = true;
            self.z_speed = 0.0;
        } else if ctx.scene.camera.z > 1.0 {
            ctx.scene.camera.z = 1.0;
            self.z_speed = 0.0;
        }

        self.z_speed -= self.gravity * delta_time;
    }

    fn wishdir(look_dir: Vector, input: Input) -> Vector {
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

    fn accelerate(&mut self, ctx: &UpdateContext, wishdir: Vector, wishspeed: f32) {
        let delta_time = ctx.delta_time.as_secs_f32();
        self.apply_friction(delta_time);

        let currentspeed = self.velocity.dot_product(wishdir);
        let addspeed = wishspeed - currentspeed;
        if addspeed <= 0.0 {
            return;
        }

        let mut accelspeed = self.acceleration * wishspeed * delta_time;
        if accelspeed > addspeed {
            accelspeed = addspeed;
        }

        self.velocity += accelspeed * wishdir;
    }

    fn apply_friction(&mut self, delta_time: f32) {
        let speed = self.velocity.mag();
        if speed < 0.01 {
            self.velocity = Vector::ZERO;
            return;
        }

        let control = if speed < self.stop_speed {
            self.stop_speed
        } else {
            speed
        };

        let drop = control * self.friction * delta_time;
        let mut new_speed = speed - drop;
        if new_speed < 0.0 {
            new_speed = 0.0;
        }
        new_speed /= speed;

        self.velocity *= new_speed;
    }
}

impl Script for Q1Controller {
    fn start(&mut self, ctx: StartContext) {
        ctx.scene.camera.z = self.height;
        ctx.system.set_capture_mouse(true);
    }

    fn tick(&mut self, mut ctx: UpdateContext) {
        let delta_time = ctx.delta_time.as_secs_f32();
        if (ctx.input.was_key_pressed(Scancode::Space) || ctx.input.is_key_down(Scancode::Space))
            && self.grounded
        {
            self.z_speed = self.jump_speed;
            self.grounded = false;
        }

        self.update_velocity(&ctx);
        self.update_z(&mut ctx, delta_time);

        let mouse_delta = ctx.input.mouse_rel().0;
        ctx.scene
            .camera
            .rotate(self.sensitivity * -0.022 * mouse_delta as f32);

        ctx.scene.camera.ray.translate(self.velocity * delta_time);
    }

    fn end(&mut self, _ctx: EndContext) {}
}

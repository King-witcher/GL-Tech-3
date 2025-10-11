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
    pub m_sensitivity: f32,
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
            acceleration: 10.0,
            air_acceleration: 7.0,
            jump_speed: 270.0,
            gravity: 800.0,
            m_sensitivity: 2.2,
            friction: 6.0,
            height: 46.0,
            max_speed: 320.0,
            stop_speed: 100.0,

            velocity: Vector::ZERO,
            grounded: true,
            z_speed: 0.0,
        }
    }
}

impl Q1Controller {
    /// Update horizontal velocity based on input
    fn update_velocity(&mut self, ctx: &UpdateContext) {
        let wishdir = Self::wishdir(ctx.scene.camera.ray.dir, ctx.input.clone());

        if self.grounded {
            self.accelerate(ctx, wishdir, self.max_speed);
        } else {
            self.air_accelerate(ctx, wishdir, self.max_speed);
        }
    }

    /// Check for jump input and initiate jump if grounded
    fn check_jump(&mut self, ctx: &UpdateContext) {
        if (ctx.input.was_key_pressed(Scancode::Space) || ctx.input.is_key_down(Scancode::Space))
            && self.grounded
        {
            self.z_speed = self.jump_speed;
            self.grounded = false;
        }
    }

    /// Update the vertical position based on z_speed
    fn update_z(&mut self, ctx: &mut UpdateContext) {
        if self.grounded {
            return;
        }

        let delta_time = ctx.delta_time.as_secs_f32();
        ctx.scene.camera.z += self.z_speed * delta_time;
        if ctx.scene.camera.z < self.height {
            ctx.scene.camera.z = self.height;
            self.grounded = true;
            self.z_speed = 0.0;
        } else if ctx.scene.camera.z > 100.0 {
            ctx.scene.camera.z = 100.0;
            self.z_speed = 0.0;
        }

        self.z_speed -= self.gravity * delta_time;
    }

    /// Update the view direction based on mouse movement
    fn update_view(&mut self, ctx: &mut UpdateContext) {
        let mouse_delta = ctx.input.mouse_rel().0;
        ctx.scene
            .camera
            .rotate(self.m_sensitivity * -0.022 * mouse_delta as f32);
    }

    /// Calculate the desired movement direction based on input
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

    /// Accelerate the player in the desired direction
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

    fn air_accelerate(&mut self, ctx: &UpdateContext, wishdir: Vector, wishspeed: f32) {
        let delta_time = ctx.delta_time.as_secs_f32();

        let wishspd = f32::min(wishspeed, 0.3);
        let currentspeed = self.velocity.dot_product(wishdir);
        let addspeed = wishspd - currentspeed;

        if addspeed <= 0.0 {
            return;
        }

        let mut accelspeed = f32::min(self.air_acceleration * wishspeed * delta_time, addspeed);
        self.velocity += accelspeed * wishdir;
    }

    /// Apply friction to the player's velocity
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

        self.velocity *= new_speed / speed;
    }
}

impl Script for Q1Controller {
    fn start(&mut self, ctx: StartContext) {
        ctx.scene.camera.z = self.height;
        ctx.system.set_capture_mouse(true);
    }

    fn tick(&mut self, mut ctx: UpdateContext) {
        self.update_view(&mut ctx);

        self.check_jump(&ctx);
        self.update_z(&mut ctx);

        self.update_velocity(&ctx);
        ctx.scene
            .camera
            .ray
            .translate(self.velocity * ctx.delta_time.as_secs_f32());
    }

    fn end(&mut self, _ctx: EndContext) {}
}

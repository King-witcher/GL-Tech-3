use crate::prelude::*;
use std::f32;

use crate::{Image, Scene};

pub fn draw_scene(scene: &Scene, image: &mut Image) {
    let (width, height) = image.dimensions();
    unsafe {
        std::ptr::write_bytes(image.u32_buffer(), 0, (width * height) as usize);
    }
    let tan = (110.0 * 0.5 * f32::consts::PI / 180.0).tan();
    let step0 = 2.0 * tan / image.widthf;
    let col_height_1 = image.widthf / (2.0 * tan);
    let camera_dir = scene.camera.dir();
    let camera_left = Vector(-camera_dir.1, camera_dir.0);

    for col in 0..width {
        let ray = {
            let delta = (width >> 1) as i32 - col as i32;
            let dir = camera_dir + camera_left * step0 * delta as f32;
            Ray::new(scene.camera.pos(), dir)
        };

        let collision = scene.raycast(ray);

        let Some((plane, (distance, u))) = collision else {
            continue;
        };

        let col_h = col_height_1 / (ray.dir.dot_product(camera_dir) * distance);
        let col_start = (image.heightf - 1.0 - col_h) * 0.5;
        let col_end = (image.heightf - 1.0 + col_h) * 0.5;

        let mut draw_col_start = height as i32 - (image.heightf - col_start) as i32; // Inclusive
        let mut draw_col_end = height as i32 - (image.heightf - col_end) as i32; // Exclusive

        if draw_col_start < 0 {
            draw_col_start = 0;
        }

        if draw_col_end >= height as i32 {
            draw_col_end = height as i32;
        }

        let i_col_h = 1.0 / col_h;
        for line in draw_col_start..draw_col_end {
            let v = (line as f32 - col_start) * i_col_h;
            let color = plane.texture.map_nearest(u, v);
            image.set(col as u32, line as u32, color);
        }
    }
}

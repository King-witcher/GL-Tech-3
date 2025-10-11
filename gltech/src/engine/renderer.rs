use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{Camera, Plane, prelude::*};
use std::f32;

use crate::Image;

pub fn draw_planes(camera: &Camera, planes: Vec<&Plane>, image: &Image) {
    let (width, height) = image.dimensions();
    unsafe {
        std::ptr::write_bytes(image.u32_buffer(), 0, (width * height) as usize);
    }
    let tan = (camera.fov * 0.5 * f32::consts::PI / 180.0).tan();
    let step0 = 2.0 * tan / image.widthf;
    let col_height_1 = image.widthf / (2.0 * tan);
    let camera_pos = camera.pos();
    let camera_dir = camera.dir();
    let camera_left = Vector(-camera_dir.1, camera_dir.0);

    (0..width).into_par_iter().for_each(|col| {
        let (width, height) = image.dimensions();
        let ray = {
            let delta = (width >> 1) as i32 - col as i32;
            let dir = camera_dir + camera_left * step0 * delta as f32;
            Ray::new(camera_pos, dir)
        };

        let Some((plane, (collision_r, collision_s))) = get_nearest(&planes, ray) else {
            return;
        };

        let col_h = col_height_1 / (ray.dir.dot_product(camera_dir) * collision_r);
        let col_start = (image.heightf - 1.0 - col_h) * 0.5 + col_h * (camera.z - 0.5);
        let col_end = (image.heightf - 1.0 + col_h) * 0.5 + col_h * (camera.z - 0.5);

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
            let color = plane.texture.map_nearest(collision_s, v);
            image.set_unsafe(col as u32, line as u32, color);
        }
    });
}

fn get_nearest<'a>(planes: &Vec<&'a Plane>, ray: Ray) -> Option<(&'a Plane, (f32, f32))> {
    let mut rs = (f32::INFINITY, f32::INFINITY);
    let mut nearest_plane = None;

    for plane in planes {
        let (distance, split) = ray.get_rs(plane.segment);

        if distance < 0.0 || split < 0.0 || split >= 1.0 {
            continue;
        };

        if distance < rs.0 {
            rs = (distance, split);
            nearest_plane = Some(plane);
        }
    }

    Some((nearest_plane?, rs))
}

use std::{cell::RefCell, ptr::NonNull, rc::Rc};

use math::Pose;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    Image, Plane,
    data::{ViewInfo, WeakList},
};

pub struct Renderer2 {
    weak_planes: WeakList<Plane>,
    strong_planes: Vec<Rc<RefCell<Plane>>>,
    frame_buffer: Image,
    view_info: ViewInfo,
}

impl Renderer2 {
    pub fn new(width: i32, height: i32) -> Self {
        let frame = Image::new(width, height);
        Self {
            weak_planes: WeakList::new(),
            strong_planes: Vec::new(),
            frame_buffer: frame,
            view_info: ViewInfo::from_fov(70.0, 640.0),
        }
    }

    pub fn add_plane(&mut self, plane: Plane) -> Rc<RefCell<Plane>> {
        let rc_plane = Rc::new(RefCell::new(plane));
        let ptr = NonNull::new(rc_plane.as_ptr()).unwrap();

        self.weak_planes.push(ptr);
        self.strong_planes.push(rc_plane.clone());

        rc_plane
    }

    pub fn clear(&mut self) {
        unsafe {
            std::ptr::write_bytes(
                self.frame_buffer.byte_slice() as *const _ as *mut u32,
                0,
                (self.frame_buffer.width() * self.frame_buffer.height()) as usize,
            );
        }
    }

    pub fn render(&mut self, view: math::Pose, z: f32) {
        let frame_buffer = &mut self.frame_buffer;
        let left = view.dir.left();

        let height_f = frame_buffer.heightf;

        (0..frame_buffer.width())
            .into_par_iter()
            .for_each(|col_idx| {
                let (width, height) = frame_buffer.dimensions();
                let ray = {
                    let delta = (width >> 1) - col_idx;
                    let dir = view.dir + left * self.view_info.step_0 * delta as f32;
                    Pose::new(view.pos, dir)
                };

                let Some((plane, (collision_r, collision_s))) = get_nearest(&self.weak_planes, ray)
                else {
                    return;
                };

                let collision_depth = ray.dir.dot_product(view.dir) * collision_r;
                let col_height_f = self.view_info.col_height_1 / collision_depth;
                let col_start_f = (height_f - 1.0 - col_height_f) * 0.5 + col_height_f * (z - 0.5);
                let col_end_f = (height_f - 1.0 + col_height_f) * 0.5 + col_height_f * (z - 0.5);

                let mut col_start_i = height - (height_f - col_start_f) as i32; // Inclusive
                col_start_i = col_start_i.max(0);

                let mut col_end_i = height - (height_f - col_end_f) as i32; // Exclusive
                col_end_i = col_end_i.min(height as i32);

                let i_col_h = 1.0 / col_height_f;
                for line_idx in col_start_i..col_end_i {
                    let v = (line_idx as f32 - col_start_f) * i_col_h;
                    let color = plane.texture.map_nearest(collision_s, v);
                    frame_buffer.set_unsafe(col_idx, line_idx, color);
                }
            });
    }

    #[inline]
    pub fn frame_buffer(&self) -> Image {
        self.frame_buffer.cheap_clone()
    }
}

fn get_nearest(planes: &WeakList<Plane>, ray: Pose) -> Option<(&Plane, (f32, f32))> {
    let mut rs = (f32::INFINITY, f32::INFINITY);
    let mut nearest_plane = None;

    for plane in planes.iter() {
        let (distance, split) = ray.get_rs(unsafe { plane.as_ref() }.pose);

        if distance < 0.0 || split < 0.0 || split >= 1.0 {
            continue;
        };

        if distance < rs.0 {
            rs = (distance, split);
            nearest_plane = Some(unsafe { plane.as_ref() });
        }
    }

    Some((nearest_plane?, rs))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Image, TextureClip};
    use math::Vector;

    fn generate_plane() -> Plane {
        let image = Image::new(1, 1);
        Plane::new(
            Vector(0.0, 0.0),
            Vector(1.0, 0.0),
            TextureClip::new(crate::TextureClipCreateInfo {
                image,
                hoffset: 0.0,
                voffset: 0.0,
                hrepeat: 1.0,
                vrepeat: 1.0,
            }),
        )
    }

    #[test]
    fn test_renderer_add_plane() {
        let mut renderer = Renderer2::new(1920, 1080);
        renderer.add_plane(generate_plane());
        renderer.add_plane(generate_plane());
        renderer.add_plane(generate_plane());
        renderer.add_plane(generate_plane());

        drop(renderer);
    }
}

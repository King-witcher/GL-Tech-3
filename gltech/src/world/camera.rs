use crate::prelude::*;

pub struct Camera {
    pub ray: Ray,
    pub z: f32,
    pub fov: f32,
}

impl Camera {
    #[inline]
    pub fn pos(&self) -> Vector {
        self.ray.start
    }

    #[inline]
    pub fn dir(&self) -> Vector {
        self.ray.dir
    }

    #[inline]
    pub fn set_pos(&mut self, pos: impl Into<Vector>) {
        self.ray.set_pos(pos.into());
    }

    #[inline]
    pub fn set_dir(&mut self, dir: impl Into<Vector>) {
        self.ray.set_dir(dir.into());
    }

    #[inline]
    pub fn translate(&mut self, delta: impl Into<Vector>) {
        self.ray.translate(delta.into());
    }

    #[inline]
    pub fn transform(&mut self, transformation: impl Into<Vector>) {
        self.ray.transform(transformation.into());
    }

    #[inline]
    pub fn angle(&self) -> f32 {
        self.dir().angle()
    }

    #[inline]
    pub fn set_angle(&mut self, angle: f32) {
        let new_dir = Vector::from_deg(angle);
        self.set_dir(new_dir);
    }

    #[inline]
    pub fn rotate(&mut self, angle: f32) {
        let dir = self.dir();
        let new_dir = dir.cmul(Vector::from_deg(angle));
        self.set_dir(new_dir);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            ray: Ray::new(Vector::ZERO, Vector::FORWARD),
            z: 50.0,
            fov: 90.0,
        }
    }
}

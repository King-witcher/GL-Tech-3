use crate::imaging::Texture;
use crate::prelude::*;

pub struct Plane {
    pub segment: Ray,
    pub texture: Texture,
}

impl Plane {
    pub fn new(start: Vector, dir: Vector, texture: Texture) -> Self {
        Self {
            segment: Ray::new(start, dir),
            texture,
        }
    }

    #[inline]
    pub fn end(&self) -> Vector {
        self.segment.end()
    }
}

impl Spatial for Plane {
    #[inline]
    fn pos(&self) -> Vector {
        self.segment.start
    }

    #[inline]
    fn set_pos(&mut self, pos: Vector) {
        self.segment.set_pos(pos);
    }

    #[inline]
    fn dir(&self) -> Vector {
        self.segment.dir()
    }

    #[inline]
    fn set_dir(&mut self, dir: Vector) {
        self.segment.set_dir(dir);
    }

    #[inline]
    fn translate(&mut self, delta: Vector) {
        self.segment.translate(delta);
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plane <{}, {}>", self.segment.start, self.segment.end())
    }
}

use crate::imaging::Texture;
use crate::prelude::*;

use crate::world::Entity;

pub struct Plane<'a> {
    pub segment: Ray,
    pub texture: Texture<'a>,
}

impl<'a> Plane<'a> {
    pub fn new(start: Vector, dir: Vector, texture: Texture<'a>) -> Self {
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

impl Spatial for Plane<'_> {
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

impl<'a> From<Plane<'a>> for Entity<'a> {
    #[inline]
    fn from(plane: Plane<'a>) -> Self {
        Entity::from_plane(plane)
    }
}

impl std::fmt::Display for Plane<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plane <{}, {}>", self.segment.start, self.segment.end())
    }
}

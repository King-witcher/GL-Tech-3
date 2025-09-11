use crate::prelude::*;

use crate::world::{Entity, Spatial};

pub struct Plane {
    // pub(crate) inner: Box<PlaneInner>,
    pub start: Vector,
    pub direction: Vector,
}

pub(crate) struct RayHit {
    pub r: f32,
    pub s: f32,
}

impl Plane {
    pub fn new(start: Vector, direction: Vector) -> Self {
        Self { start, direction }
    }

    pub(crate) fn test_ray(&self, origin: Vector, direction: Vector) -> Option<RayHit> {
        let det = direction.x() * self.direction.y() - direction.y() * self.direction.x();
        let idet = 1.0 / det;

        // Parallel lines
        if det == 0.0 {
            return None;
        }

        let spldet = direction.x() * (origin.y() - self.start.y())
            - direction.y() * (origin.x() - self.start.x());
        let dstdet = self.direction.x() * (origin.y() - self.start.y())
            - self.direction.y() * (origin.x() - self.start.x());
        let spltmp = spldet * idet;
        let dsttmp = dstdet * idet;

        if spltmp < 0.0 || spltmp >= 1.0 || dsttmp <= 0.0 {
            return None;
        }

        Some(RayHit { r: dsttmp, s: 0.5 })
    }

    #[inline]
    pub fn end(&self) -> Vector {
        self.start + self.direction
    }
}

impl Spatial for Plane {
    #[inline]
    fn pos(&self) -> Vector {
        self.start
    }

    #[inline]
    fn set_pos(&mut self, pos: Vector) {
        self.start = pos;
    }

    #[inline]
    fn dir(&self) -> Vector {
        self.direction
    }

    #[inline]
    fn set_dir(&mut self, dir: Vector) {
        self.direction = dir;
    }

    #[inline]
    fn translate(&mut self, delta: Vector) {
        self.start = self.start + delta;
    }

    #[inline]
    fn transform(&mut self, by: Vector) {
        self.direction = self.direction.cmul(by);
    }
}

impl From<Plane> for Entity {
    #[inline]
    fn from(plane: Plane) -> Self {
        Entity::from_plane(plane)
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plane <{:?}, {:?}>",
            self.start,
            self.start + self.direction
        )
    }
}

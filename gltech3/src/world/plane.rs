use crate::prelude::*;

use crate::world::{Entity, Spatial};

pub struct Plane {
    // pub(crate) inner: Box<PlaneInner>,
    pub segment: Segment,
}

impl Plane {
    pub fn new(start: Vector, dir: Vector) -> Self {
        Self {
            segment: Segment::new(start, dir),
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
        self.segment.start = pos;
    }

    #[inline]
    fn dir(&self) -> Vector {
        self.segment.dir
    }

    #[inline]
    fn set_dir(&mut self, dir: Vector) {
        self.segment.dir = dir;
    }

    #[inline]
    fn translate(&mut self, delta: Vector) {
        self.segment.start = self.segment.start + delta;
    }

    #[inline]
    fn transform(&mut self, by: Vector) {
        self.segment.dir = self.segment.dir.cmul(by);
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
            self.segment.start,
            self.segment.end()
        )
    }
}

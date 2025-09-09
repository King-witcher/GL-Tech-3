use crate::{
    vector::Vector,
    world::{Entity, EntityNode},
};

pub(crate) struct PlaneInner {}

pub struct Plane {
    // pub(crate) inner: Box<PlaneInner>,
    pub start: Vector,
    pub direction: Vector,
}

pub(crate) struct RayHit {
    pub distance: f32,
    pub color: u32,
}

impl Plane {
    pub fn new(start: Vector, direction: Vector) -> Self {
        Self { start, direction }
    }

    pub(crate) fn test_ray(&self, origin: Vector, direction: Vector) -> Option<RayHit> {
        let det = direction.0 * self.direction.1 - direction.1 * self.direction.0;
        let idet = 1.0 / det;
        if det == 0.0 {
            return None;
        }

        let spldet =
            direction.0 * (origin.1 - self.start.1) - direction.1 * (origin.0 - self.start.0);
        let dstdet = self.direction.0 * (origin.1 - self.start.1)
            - self.direction.1 * (origin.0 - self.start.0);
        let spltmp = spldet * idet;
        let dsttmp = dstdet * idet;

        if spltmp < 0.0 || spltmp >= 1.0 || dsttmp <= 0.0 {
            return None;
        }

        Some(RayHit {
            distance: dsttmp,
            color: 0xFFFFFFFF,
        })
    }

    #[inline]
    pub fn end(&self) -> Vector {
        self.start + self.direction
    }
}

impl Entity for Plane {
    #[inline]
    fn pos(&self) -> Vector {
        self.start
    }

    #[inline]
    fn transform(&self) -> Vector {
        self.direction
    }
}

impl From<Plane> for EntityNode {
    #[inline]
    fn from(plane: Plane) -> Self {
        EntityNode::from_plane(plane)
    }
}

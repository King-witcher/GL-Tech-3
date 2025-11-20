use math::{Pose, Posed, Vector};

use crate::TextureClip;

pub struct Plane {
    pub pose: Pose,
    pub texture: TextureClip,
}

impl Plane {
    pub fn new(start: Vector, dir: Vector, texture: TextureClip) -> Self {
        Self {
            pose: Pose::new(start, dir),
            texture,
        }
    }

    #[inline]
    pub fn end(&self) -> Vector {
        self.pose.end()
    }
}

// TODO: Optimize
impl Posed for Plane {
    #[inline]
    fn pose(&self) -> Pose {
        self.pose
    }

    fn set_pose(&mut self, pose: Pose) {
        self.pose = pose;
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plane <{}, {}>", self.pose.pos, self.pose.end())
    }
}

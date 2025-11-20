use crate::TextureClip;
use math::Pose;
use math::Vector;

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
}

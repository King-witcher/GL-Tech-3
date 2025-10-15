use crate::prelude::*;

pub struct Camera {
    pub ray: Pose,
    pub z: f32,
    pub fov: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            ray: Pose::new(Vector::ZERO, Vector::FORWARD),
            z: 50.0,
            fov: 90.0,
        }
    }
}

// TODO: Optimize
impl Posed for Camera {
    #[inline]
    fn pose(&self) -> Pose {
        self.ray
    }

    fn set_pose(&mut self, pose: Pose) {
        self.ray = pose;
    }
}

use crate::prelude::*;

/// Represents an object that has a position and direction/rotation in 2D space.
pub trait Spatial {
    fn pos(&self) -> Vector;
    fn dir(&self) -> Vector;
    fn set_pos(&mut self, pos: Vector);
    fn set_dir(&mut self, dir: Vector);

    #[inline]
    fn angle(&self) -> f32 {
        self.dir().angle()
    }
    #[inline]
    fn set_angle(&mut self, angle: f32) {
        let trans = Vector::from_angle(angle);
        let dir = self.dir();
        self.set_dir(dir.cmul(trans));
    }

    // This method can be optimized using assign operations.
    #[inline]
    fn translate(&mut self, delta: Vector) {
        self.set_pos(self.pos() + delta);
    }
    #[inline]
    fn transform(&mut self, transformation: Vector) {
        let dir = self.dir();
        self.set_dir(dir.cmul(transformation));
    }
    #[inline]
    fn rotate(&mut self, angle: f32) {
        let trans = Vector::from_angle(angle);
        let dir = self.dir();
        self.set_dir(dir.cmul(trans));
    }
}

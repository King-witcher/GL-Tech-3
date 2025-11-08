use crate::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Pose {
    pub pos: Vector,
    pub dir: Vector,
}

impl Pose {
    #[inline]
    pub fn new(pos: impl Into<Vector>, dir: impl Into<Vector>) -> Self {
        Self {
            pos: pos.into(),
            dir: dir.into(),
        }
    }

    #[inline]
    pub fn end(&self) -> Vector {
        self.pos + self.dir
    }

    #[inline]
    pub fn set_end(&mut self, end: Vector) {
        self.dir = end - self.pos;
    }

    /// Returns the parameters `r` and `s` such that:
    ///
    /// `self.start + r * self.dir == other.start + s * other.dir`
    /// If the rays are parallel, returns `(f32::INFINITY, f32::INFINITY)`.
    #[inline]
    pub fn get_rs(self, other: Pose) -> (f32, f32) {
        let det = self.dir.y() * other.dir.x() - self.dir.x() * other.dir.y();
        if det == 0.0 {
            return (f32::INFINITY, f32::INFINITY);
        }

        let idet = 1.0 / det;
        let delta = other.pos - self.pos;

        let r = idet * (other.dir.x() * delta.y() - other.dir.y() * delta.x());
        let s = idet * (self.dir.x() * delta.y() - self.dir.y() * delta.x());

        (r, s)
    }

    #[inline]
    pub fn as_relative_to(self, reference: Pose) -> Pose {
        Pose {
            pos: (self.pos - reference.pos).cdiv(reference.dir),
            dir: self.dir.cdiv(reference.dir),
        }
    }

    #[inline]
    pub fn as_absolute_from(self, reference: Pose) -> Pose {
        Pose {
            pos: reference.pos + self.pos.cmul(reference.dir),
            dir: self.dir.cmul(reference.dir),
        }
    }
}

/// Represents an object that has a position and direction/rotation in 2D space.
pub trait Posed {
    fn pose(&self) -> Pose;
    fn set_pose(&mut self, pose: Pose);

    #[inline]
    fn pos(&self) -> Vector {
        self.pose().pos
    }
    #[inline]
    fn dir(&self) -> Vector {
        self.pose().dir
    }
    #[inline]
    fn angle(&self) -> f32 {
        self.dir().angle()
    }

    // These methods can be optimized using direct access.
    #[inline]
    fn set_pos(&mut self, pos: Vector) {
        let mut pose = self.pose();
        pose.pos = pos;
        self.set_pose(pose);
    }
    #[inline]
    fn set_dir(&mut self, dir: Vector) {
        let mut pose = self.pose();
        pose.dir = dir;
        self.set_pose(pose);
    }
    #[inline]
    fn set_angle(&mut self, angle_deg: f32) {
        let mut pose = self.pose();
        let new_dir = Vector::from_deg(angle_deg) * pose.dir.mag();
        pose.dir = new_dir;
        self.set_pose(pose);
    }

    // This method can be optimized using assign operations.
    #[inline]
    fn r#move(&mut self, delta: Vector) {
        self.set_pos(self.pos() + delta);
    }
    #[inline]
    fn transform(&mut self, transformation: Vector) {
        let dir = self.dir();
        self.set_dir(dir.cmul(transformation));
    }
    #[inline]
    fn rotate(&mut self, angle: f32) {
        let trans = Vector::from_deg(angle);
        let dir = self.dir();
        self.set_dir(dir.cmul(trans));
    }
}

// TODO: Optimize
impl Posed for Pose {
    #[inline]
    fn pose(&self) -> Pose {
        *self
    }

    fn set_pose(&mut self, pose: Pose) {
        *self = pose;
    }
}

impl Default for Pose {
    fn default() -> Self {
        Self {
            pos: Vector::ZERO,
            dir: Vector::EAST,
        }
    }
}

impl std::fmt::Display for Pose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ray <{} -> {}>", self.pos, self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(a: f32, b: f32) {
        let diff = (a - b).abs();
        assert!(diff < 1e-3, "{} is not approximately equal to {}", a, b);
    }

    #[test]
    fn test_rs_controlled() {
        let seg1 = Pose::new(Vector(-1.0, 0.0), Vector(1.0, 1.0));
        let seg2 = Pose::new(Vector(1.0, 0.0), Vector(-1.0, 1.0));

        let (r, s) = seg1.get_rs(seg2);
        let col1 = seg1.pos + seg1.dir * r;
        let col2 = seg2.pos + seg2.dir * s;

        let distance = (col1 - col2).mag();

        assert_eq!(distance, 0.0);
    }

    #[test]
    fn test_rs_random() {
        for _ in 0..10 {
            let seg1 = Pose::new(
                Vector(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                ),
                Vector(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                ),
            );
            let seg2 = Pose::new(
                Vector(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                ),
                Vector(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                ),
            );

            let (r, s) = seg1.get_rs(seg2);

            let collision1 = seg1.pos + seg1.dir * r;
            let collision2 = seg2.pos + seg2.dir * s;

            let distance = (collision1 - collision2).mag();

            if r.is_finite() && s.is_finite() {
                assert_approx_eq(distance, 0.0);
            }
        }
    }
}

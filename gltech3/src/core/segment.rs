use crate::Vector;

#[derive(Clone, Copy)]
struct Segment {
    pub start: Vector,
    pub dir: Vector,
}

impl Segment {
    #[inline]
    pub fn new(start: Vector, dir: Vector) -> Self {
        Self { start, dir }
    }

    #[inline]
    fn get_rs(self, other: Segment) -> Vector {
        let det = self.dir.x() * other.dir.y() - self.dir.y() * other.dir.x();
        if det == 0.0 {
            return Vector(f32::INFINITY, f32::INFINITY);
        }

        let idet = 1.0 / det;
        let start_distance = other.start - self.start;

        let r = idet * (other.dir.y() * start_distance.x() - self.dir.y() * start_distance.y());
        let s = idet * (self.dir.x() * start_distance.y() - other.dir.x() * start_distance.x());

        Vector(r, s)
    }
}

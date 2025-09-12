use crate::Vector;

#[derive(Clone, Copy)]
pub struct Segment {
    pub start: Vector,
    pub dir: Vector,
}

impl Segment {
    #[inline]
    pub fn new(start: Vector, dir: Vector) -> Self {
        Self { start, dir }
    }

    #[inline]
    pub fn end(&self) -> Vector {
        self.start + self.dir
    }

    #[inline]
    pub fn get_rs(self, other: Segment) -> (f32, f32) {
        let det = self.dir.y() * other.dir.x() - self.dir.x() * other.dir.y();
        if det == 0.0 {
            return (f32::INFINITY, f32::INFINITY);
        }

        let idet = 1.0 / det;
        let delta = other.start - self.start;

        let r = idet * (other.dir.x() * delta.y() - other.dir.y() * delta.x());
        let s = idet * (self.dir.x() * delta.y() - self.dir.y() * delta.x());

        (r, s)
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
        let seg1 = Segment::new(Vector(-1.0, 0.0), Vector(1.0, 1.0));
        let seg2 = Segment::new(Vector(1.0, 0.0), Vector(-1.0, 1.0));

        let (r, s) = seg1.get_rs(seg2);
        let col1 = seg1.start + seg1.dir * r;
        let col2 = seg2.start + seg2.dir * s;

        let distance = (col1 - col2).mag();

        assert_eq!(distance, 0.0);
    }

    #[test]
    fn test_rs_random() {
        for _ in 0..10 {
            let seg1 = Segment::new(
                Vector(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                ),
                Vector(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                ),
            );
            let seg2 = Segment::new(
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

            let collision1 = seg1.start + seg1.dir * r;
            let collision2 = seg2.start + seg2.dir * s;

            let distance = (collision1 - collision2).mag();

            if r.is_finite() && s.is_finite() {
                assert_approx_eq(distance, 0.0);
            }
        }
    }
}

use std::{fmt::Display, ops::*};

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub fn from_deg(angle_deg: f32) -> Vector {
        let rad = angle_deg * TO_RAD;
        Vector(rad.cos(), rad.sin())
    }

    #[inline]
    pub fn mag(&self) -> f32 {
        f32::sqrt(self.x() * self.x() + self.y() * self.y())
    }

    pub fn modularize(&mut self) -> f32 {
        let module = self.mag();
        let imodule = 1.0 / module;
        if module != 0.0 {
            self.0 *= imodule;
            self.1 *= imodule;
        }
        module
    }

    #[inline]
    pub fn dot_product(self, other: Vector) -> f32 {
        self.x() * other.x() + self.y() * other.y()
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn cmul(self, by: Vector) -> Vector {
        Vector(
            self.x() * by.x() - self.y() * by.y(),
            self.x() * by.y() + self.y() * by.x(),
        )
    }

    #[inline]
    pub fn cdiv(self, by: Vector) -> Vector {
        let i_squares_sum = 1.0 / (by.x() * by.x() + by.y() * by.y());
        Vector(
            (self.x() * by.x() + self.y() * by.y()) * i_squares_sum,
            (self.y() * by.x() - self.x() * by.y()) * i_squares_sum,
        )
    }

    #[inline]
    pub fn angle(self) -> f32 {
        if self.x() == 0.0 && self.y() == 0.0 {
            return 0.0;
        }

        let temp = TO_DEG * f32::acos(self.x() / self.mag());
        if self.y() >= 0.0 { temp } else { 360.0 - temp }
    }

    // TODO: Optimize
    pub fn set_rotation(&mut self, angle: f32) {
        let length = self.mag();
        self.0 = length * angle.to_radians().cos();
        self.1 = length * angle.to_radians().sin();
    }
}

// Constants

impl Vector {
    pub const ZERO: Vector = Vector(0.0, 0.0);
    pub const IDENTITY: Vector = Vector(1.0, 1.0);
    pub const FORWARD: Vector = Vector(1.0, 0.0);
    pub const RIGHT: Vector = Vector(0.0, -1.0);
    pub const LEFT: Vector = Vector(0.0, 1.0);
    pub const BACK: Vector = Vector(-1.0, 0.0);
}

impl Add for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, other: Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Vector {
    #[inline]
    fn add_assign(&mut self, other: Vector) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Sub for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, other: Vector) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for Vector {
    #[inline]
    fn sub_assign(&mut self, other: Vector) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, scalar: f32) -> Vector {
        Vector(self.0 * scalar, self.1 * scalar)
    }
}
impl Mul<Vector> for f32 {
    type Output = Vector;

    #[inline]
    fn mul(self, vector: Vector) -> Vector {
        Vector(self * vector.0, self * vector.1)
    }
}

impl MulAssign<f32> for Vector {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.0 *= scalar;
        self.1 *= scalar;
    }
}

impl Mul for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.cmul(rhs)
    }
}

// impl Mul for Vector {
//     type Output = f32;

//     #[inline]
//     fn mul(self, rhs: Self) -> Self::Output {
//         Vector::dot_product(&self, &rhs)
//     }
// }

impl Div<f32> for Vector {
    type Output = Vector;

    #[inline]
    fn div(self, scalar: f32) -> Vector {
        let i_scalar = 1.0 / scalar;
        Vector(self.0 * i_scalar, self.1 * i_scalar)
    }
}

impl DivAssign<f32> for Vector {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        let i_scalar = 1.0 / scalar;
        self.0 *= i_scalar;
        self.1 *= i_scalar;
    }
}

impl Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Vector {
        Vector(-self.0, -self.1)
    }
}

impl From<(f32, f32)> for Vector {
    #[inline]
    fn from(tuple: (f32, f32)) -> Self {
        Vector(tuple.0, tuple.1)
    }
}

const TO_RAD: f32 = std::f32::consts::PI / 180.0;
const TO_DEG: f32 = 180.0 / std::f32::consts::PI;

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EPSILON: f32 = 0.0001;

    fn assert_aproximation(value: f32, expected: f32) {
        assert!(
            (value - expected).abs() < EPSILON,
            "Expected approximately {}, but got {}",
            expected,
            value
        );
    }

    #[test]
    fn angle() {
        assert_aproximation(Vector(1.0, 0.0).angle(), 0.0);
        assert_aproximation(Vector(1.0, 1.0).angle(), 45.0);
        assert_aproximation(Vector(0.0, 1.0).angle(), 90.0);
        assert_aproximation(Vector(-1.0, 1.0).angle(), 135.0);
        assert_aproximation(Vector(-1.0, 0.0).angle(), 180.0);
        assert_aproximation(Vector(-1.0, -1.0).angle(), 225.0);
        assert_aproximation(Vector(0.0, -1.0).angle(), 270.0);
        assert_aproximation(Vector(1.0, -1.0).angle(), 315.0);
    }

    #[test]
    fn modularize() {
        let mut v = Vector(3.0, 4.0);
        let module = v.modularize();
        assert_aproximation(module, 5.0);
        assert_aproximation(v.mag(), 1.0);
    }

    #[test]
    fn cmul() {
        let first = Vector(rand::random(), rand::random());
        let second = Vector(rand::random(), rand::random());

        let result = first.cmul(second);

        assert_aproximation(result.mag(), first.mag() * second.mag());
        assert_aproximation(result.angle(), first.angle() + second.angle());
    }

    #[test]
    fn cdiv() {
        let first = Vector(rand::random(), rand::random());
        let second = Vector(rand::random(), rand::random());

        let result = first.cdiv(second);

        assert_aproximation(result.mag(), first.mag() / second.mag());
        assert_aproximation(result.angle(), first.angle() - second.angle());
    }
}

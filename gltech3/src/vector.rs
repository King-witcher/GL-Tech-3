use std::{fmt::Display, ops::*};

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub fn from_angle(angle_deg: f32) -> Vector {
        let rad = angle_deg * TO_RAD;
        Vector(rad.cos(), rad.sin())
    }

    #[inline]
    pub fn module(&self) -> f32 {
        f32::sqrt(self.0 * self.0 + self.1 * self.1)
    }

    pub fn modularize(&mut self) -> f32 {
        let module = self.module();
        let imodule = 1.0 / module;
        if module != 0.0 {
            self.0 *= imodule;
            self.1 *= imodule;
        }
        module
    }

    #[inline]
    pub fn dot_product(&self, other: &Vector) -> f32 {
        self.0 * other.0 + self.1 * other.1
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
    pub fn complex_product(&self, other: &Vector) -> Vector {
        Vector(
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        )
    }

    #[inline]
    pub fn rotation(&self) -> f32 {
        if self.0 == 0.0 && self.1 == 0.0 {
            return 0.0;
        }

        let temp = TO_DEG * f32::acos(self.0 / self.module());
        if self.1 >= 0.0 { temp } else { 360.0 - temp }
    }

    // TODO: Optimize
    pub fn set_rotation(&mut self, angle: f32) {
        let length = self.module();
        self.0 = length * angle.to_radians().cos();
        self.1 = length * angle.to_radians().sin();
    }
}

// Constants

impl Vector {
    #[inline]
    pub fn forward() -> Vector {
        Vector(1.0, 0.0)
    }
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

impl MulAssign<f32> for Vector {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.0 *= scalar;
        self.1 *= scalar;
    }
}

impl Mul for Vector {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vector::dot_product(&self, &rhs)
    }
}

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

pub const ZERO: Vector = Vector(0.0, 0.0);
pub const IDENTITY: Vector = Vector(1.0, 1.0);
pub const FORWARD: Vector = Vector(0.0, 1.0);
pub const LEFT: Vector = Vector(-1.0, 0.0);
pub const RIGHT: Vector = Vector(1.0, 0.0);
pub const BACK: Vector = Vector(0.0, -1.0);

const TO_RAD: f32 = std::f32::consts::PI / 180.0;
const TO_DEG: f32 = 180.0 / std::f32::consts::PI;

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
    fn test_vector_angle() {
        assert_aproximation(Vector(1.0, 0.0).rotation(), 0.0);
        assert_aproximation(Vector(1.0, 1.0).rotation(), 45.0);
        assert_aproximation(Vector(0.0, 1.0).rotation(), 90.0);
        assert_aproximation(Vector(-1.0, 1.0).rotation(), 135.0);
        assert_aproximation(Vector(-1.0, 0.0).rotation(), 180.0);
        assert_aproximation(Vector(-1.0, -1.0).rotation(), 225.0);
        assert_aproximation(Vector(0.0, -1.0).rotation(), 270.0);
        assert_aproximation(Vector(1.0, -1.0).rotation(), 315.0);
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

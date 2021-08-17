use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl V4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub fn is_point(&self) -> bool {
        self.w != 0.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn mag(&self) -> f32 {
        let mag_sq = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        f32::sqrt(mag_sq)
    }

    pub fn normalize(&self) -> Self {
        let mag = self.mag();
        V4::new(self.x / mag, self.y / mag, self.z / mag, self.w / mag)
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add for V4 {
    type Output = V4;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for V4 {
    type Output = V4;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for V4 {
    type Output = V4;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f32> for V4 {
    type Output = V4;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul<V4> for f32 {
    type Output = V4;

    fn mul(self, rhs: V4) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for V4 {
    type Output = V4;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_almost_eq_f32, assert_almost_eq_v4};

    use super::*;

    #[test]
    fn create_vector() {
        let v = V4::vector(1.0, 0.0, 0.0);
        assert!(v == V4::new(1.0, 0.0, 0.0, 0.0));
        assert!(v.is_vector());
    }

    #[test]
    fn create_point() {
        let v = V4::point(1.0, 0.0, 0.0);
        assert!(v == V4::new(1.0, 0.0, 0.0, 1.0));
        assert!(v.is_point());
    }

    #[test]
    fn add() {
        let t1 = V4::new(3.0, -2.0, 5.0, 1.0);
        let t2 = V4::new(-2.0, 3.0, 1.0, 0.0);
        assert!(t1 + t2 == V4::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn sub_two_points() {
        let p1 = V4::point(3.0, 2.0, 1.0);
        let p2 = V4::point(5.0, 6.0, 7.0);
        let p3 = p1 - p2;
        assert!(p3 == V4::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_vector_from_point() {
        let p = V4::point(3.0, 2.0, 1.0);
        let v = V4::vector(5.0, 6.0, 7.0);
        assert!(p - v == V4::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_two_vectors() {
        let v1 = V4::vector(3.0, 2.0, 1.0);
        let v2 = V4::vector(5.0, 6.0, 7.0);
        assert!(v1 - v2 == V4::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negate_v4() {
        let v = V4::new(1.0, -2.0, 3.0, -4.0);
        let neg_v = -v;
        assert!(neg_v == V4::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiply_v4_by_scalar() {
        let v = V4::new(1.0, -2.0, 3.0, -4.0);
        assert!(v * 2.0 == V4::new(2.0, -4.0, 6.0, -8.0));
        assert!(2.0 * v == V4::new(2.0, -4.0, 6.0, -8.0));
    }

    #[test]
    fn divide_v4_by_scalar() {
        let v = V4::new(2.0, -4.0, 6.0, -8.0);
        assert!(v / 2.0 == V4::new(1.0, -2.0, 3.0, -4.0));
    }

    #[test]
    fn mag() {
        let v = V4::vector(1.0, 0.0, 0.0);
        assert_eq!(v.mag(), 1.0);

        let v = V4::vector(0.0, 1.0, 0.0);
        assert_eq!(v.mag(), 1.0);

        let v = V4::vector(0.0, 0.0, 1.0);
        assert_eq!(v.mag(), 1.0);

        let v = V4::vector(1.0, 2.0, 3.0);
        assert_almost_eq_f32(v.mag(), f32::sqrt(14.0));

        let v = V4::vector(-1.0, -2.0, -3.0);
        assert_almost_eq_f32(v.mag(), f32::sqrt(14.0));
    }

    #[test]
    fn normalize() {
        let v = V4::vector(4.0, 0.0, 0.0);
        let n = v.normalize();
        assert_almost_eq_v4(n, V4::new(1.0, 0.0, 0.0, 0.0));
        assert_almost_eq_f32(n.mag(), 1.0);
    }

    #[test]
    fn dot() {
        let a = V4::vector(1.0, 2.0, 3.0);
        let b = V4::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross() {
        let a = V4::vector(1.0, 2.0, 3.0);
        let b = V4::vector(2.0, 3.0, 4.0);
        assert_almost_eq_v4(a.cross(b), V4::vector(-1.0, 2.0, -1.0));
        assert_almost_eq_v4(b.cross(a), V4::vector(1.0, -2.0, 1.0));
    }
}

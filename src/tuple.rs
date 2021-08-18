use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple<const N: usize> {
    values: [f32; N],
}

impl<const N: usize> Tuple<N> {
    pub fn new(values: [f32; N]) -> Self {
        Self { values }
    }

    pub fn mag(&self) -> f32 {
        let mut sum_sq = 0.0;
        for n in 0..N {
            sum_sq += self.values[n] * self.values[n];
        }
        f32::sqrt(sum_sq)
    }

    pub fn normalize(&self) -> Self {
        let mag = self.mag();
        let mut values = [0.0; N];
        for n in 0..N {
            values[n] = self.values[n] / mag;
        }
        Self { values }
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        let mut dot = 0.0;
        for n in 0..N {
            dot += self[n] * rhs[n];
        }
        dot
    }
}

impl Tuple<4> {
    pub fn is_point(&self) -> bool {
        self.values[3] != 0.0
    }

    pub fn is_vector(&self) -> bool {
        self.values[3] == 0.0
    }

    pub fn cross(&self, rhs: Self) -> Self {
        vector(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }
}

impl<const N: usize> Index<usize> for Tuple<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<const N: usize> IndexMut<usize> for Tuple<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<const N: usize> Default for Tuple<N> {
    fn default() -> Self {
        Self { values: [0.0; N] }
    }
}

impl<const N: usize> Add for Tuple<N> {
    type Output = Tuple<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::Output::default();
        for n in 0..N {
            result[n] = self[n] + rhs[n]
        }
        result
    }
}

impl<const N: usize> Sub for Tuple<N> {
    type Output = Tuple<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::Output::default();
        for n in 0..N {
            result[n] = self[n] - rhs[n]
        }
        result
    }
}

impl<const N: usize> Neg for Tuple<N> {
    type Output = Tuple<N>;

    fn neg(self) -> Self::Output {
        let mut result = Self::Output::default();
        for n in 0..N {
            result[n] = -self[n];
        }
        result
    }
}

impl<const N: usize> Mul<f32> for Tuple<N> {
    type Output = Tuple<N>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Self::Output::default();
        for n in 0..N {
            result[n] = self[n] * rhs;
        }
        result
    }
}

impl<const N: usize> Mul<Tuple<N>> for f32 {
    type Output = Tuple<N>;

    fn mul(self, rhs: Tuple<N>) -> Self::Output {
        let mut result = Self::Output::default();
        for n in 0..N {
            result[n] = rhs[n] * self;
        }
        result
    }
}

impl<const N: usize> Div<f32> for Tuple<N> {
    type Output = Tuple<N>;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Self::Output::default();
        for n in 0..N {
            result[n] = self[n] / rhs;
        }
        result
    }
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple<4> {
    Tuple {
        values: [x, y, z, 0.0],
    }
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple<4> {
    Tuple {
        values: [x, y, z, 1.0],
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_almost_eq_f32, assert_almost_eq_tuple};

    use super::*;

    #[test]
    fn create_vector() {
        let v = vector(1.0, 0.0, 0.0);
        assert!(v == Tuple::new([1.0, 0.0, 0.0, 0.0]));
        assert!(v.is_vector());
    }

    #[test]
    fn create_point() {
        let v = point(1.0, 0.0, 0.0);
        assert!(v == Tuple::new([1.0, 0.0, 0.0, 1.0]));
        assert!(v.is_point());
    }

    #[test]
    fn add() {
        let t1 = Tuple::new([3.0, -2.0, 5.0, 1.0]);
        let t2 = Tuple::new([-2.0, 3.0, 1.0, 0.0]);
        assert!(t1 + t2 == Tuple::new([1.0, 1.0, 6.0, 1.0]));
    }

    #[test]
    fn sub_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let p3 = p1 - p2;
        assert!(p3 == vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert!(p - v == point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert!(v1 - v2 == vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negate_v4() {
        let v = Tuple::new([1.0, -2.0, 3.0, -4.0]);
        let neg_v = -v;
        assert!(neg_v == Tuple::new([-1.0, 2.0, -3.0, 4.0]));
    }

    #[test]
    fn multiply_v4_by_scalar() {
        let v = Tuple::new([1.0, -2.0, 3.0, -4.0]);
        assert!(v * 2.0 == Tuple::new([2.0, -4.0, 6.0, -8.0]));
        assert!(2.0 * v == Tuple::new([2.0, -4.0, 6.0, -8.0]));
    }

    #[test]
    fn divide_v4_by_scalar() {
        let v = Tuple::new([2.0, -4.0, 6.0, -8.0]);
        assert!(v / 2.0 == Tuple::new([1.0, -2.0, 3.0, -4.0]));
    }

    #[test]
    fn mag() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.mag(), 1.0);

        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.mag(), 1.0);

        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.mag(), 1.0);

        let v = vector(1.0, 2.0, 3.0);
        assert_almost_eq_f32(v.mag(), f32::sqrt(14.0));

        let v = vector(-1.0, -2.0, -3.0);
        assert_almost_eq_f32(v.mag(), f32::sqrt(14.0));
    }

    #[test]
    fn normalize() {
        let v = vector(4.0, 0.0, 0.0);
        let n = v.normalize();
        assert_almost_eq_tuple(n, Tuple::new([1.0, 0.0, 0.0, 0.0]));
        assert_almost_eq_f32(n.mag(), 1.0);
    }

    #[test]
    fn dot() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_almost_eq_tuple(a.cross(b), vector(-1.0, 2.0, -1.0));
        assert_almost_eq_tuple(b.cross(a), vector(1.0, -2.0, 1.0));
    }
}

use std::ops::{Index, IndexMut, Mul};

#[cfg(test)]
use crate::lib_test::almost_eq_f32;
use crate::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    columns: [Tuple<R>; C],
}

pub type Matrix2 = Matrix<2, 2>;
pub type Matrix3 = Matrix<3, 3>;
pub type Matrix4 = Matrix<4, 4>;

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
    fn default() -> Self {
        Self {
            columns: [Tuple::default(); C],
        }
    }
}

impl<const R: usize, const C: usize> Index<(usize, usize)> for Matrix<R, C> {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.columns[index.1][index.0]
    }
}

impl<const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<R, C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.columns[index.1][index.0]
    }
}

/// Square Matrix 전용 methods
impl<const S: usize> Matrix<S, S> {
    pub fn identity() -> Self {
        let mut m = Matrix::default();
        for i in 0..S {
            m[(i, i)] = 1.0;
        }
        m
    }
}

impl Matrix4 {
    /// remove row, column from self
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix3 {
        let mut m: Matrix3 = Matrix::default();
        for r in 0..4 {
            if r == row {
                continue;
            }

            for c in 0..4 {
                if c == column {
                    continue;
                }

                let copy_row = if r < row { r } else { r - 1 };
                let copy_col = if c < column { c } else { c - 1 };
                m[(copy_row, copy_col)] = self[(r, c)];
            }
        }
        m
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        let sub = self.submatrix(row, column);
        sub.determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let v = self.minor(row, column);
        if (row + column) % 2 == 0 {
            v
        } else {
            -v
        }
    }

    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for n in 0..4 {
            det += self.cofactor(0, n) * self[(0, n)];
        }
        det
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() <= f32::EPSILON {
            None
        } else {
            let mut m: Matrix4 = Matrix::default();
            for r in 0..4 {
                for c in 0..4 {
                    let cof = self.cofactor(r, c);
                    m[(c, r)] = cof / det;
                }
            }
            Some(m)
        }
    }
}

impl Matrix3 {
    /// remove row, column from self
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix2 {
        let mut m: Matrix2 = Matrix::default();
        for r in 0..3 {
            if r == row {
                continue;
            }

            for c in 0..3 {
                if c == column {
                    continue;
                }

                let copy_row = if r < row { r } else { r - 1 };
                let copy_col = if c < column { c } else { c - 1 };
                m[(copy_row, copy_col)] = self[(r, c)];
            }
        }
        m
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        let sub = self.submatrix(row, column);
        sub.determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let v = self.minor(row, column);
        if (row + column) % 2 == 0 {
            v
        } else {
            -v
        }
    }

    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for n in 0..3 {
            det += self.cofactor(0, n) * self[(0, n)];
        }
        det
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() <= f32::EPSILON {
            None
        } else {
            let mut m: Matrix3 = Matrix::default();
            for r in 0..3 {
                for c in 0..3 {
                    let cof = self.cofactor(r, c);
                    m[(c, r)] = cof / det;
                }
            }
            Some(m)
        }
    }
}

impl Matrix2 {
    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let row = if row == 0 { 1 } else { 0 };
        let column = if column == 0 { 1 } else { 0 };
        let v = self[(row, column)];
        if (row + column) % 2 == 0 {
            v
        } else {
            -v
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() <= f32::EPSILON {
            None
        } else {
            let mut m: Matrix2 = Matrix::default();
            for r in 0..2 {
                for c in 0..2 {
                    let cof = self.cofactor(r, c);
                    m[(c, r)] = cof / det;
                }
            }
            Some(m)
        }
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(values: &[f32]) -> Self {
        let mut m = Matrix::default();
        for r in 0..R {
            for c in 0..C {
                m.columns[c][r] = values[r * C + c];
            }
        }
        m
    }

    pub fn transpose(&self) -> Matrix<C, R> {
        let mut m = Matrix::default();
        for r in 0..R {
            for c in 0..C {
                m[(c, r)] = self[(r, c)];
            }
        }
        m
    }

    #[cfg(test)]
    pub fn almost_eq(lhs: Self, rhs: Self) -> bool {
        for r in 0..R {
            for c in 0..C {
                if !almost_eq_f32(lhs[(r, c)], rhs[(r, c)]) {
                    return false;
                }
            }
        }
        true
    }
}

impl<const R: usize, const C: usize, const C2: usize> Mul<Matrix<C, C2>> for Matrix<R, C> {
    type Output = Matrix<R, C2>;

    fn mul(self, rhs: Matrix<C, C2>) -> Self::Output {
        let mut result = Matrix::default();

        for r1 in 0..R {
            for c2 in 0..C2 {
                let mut sum = 0.0;
                for c in 0..C {
                    sum += self[(r1, c)] * rhs[(c, c2)];
                }

                result[(r1, c2)] = sum;
            }
        }

        result
    }
}

impl<const R: usize, const C: usize> Mul<Tuple<C>> for Matrix<R, C> {
    type Output = Tuple<C>;

    fn mul(self, rhs: Tuple<C>) -> Self::Output {
        let mut result = Tuple::default();

        for r in 0..R {
            let mut sum = 0.0;
            for c in 0..C {
                sum += self[(r, c)] * rhs[c];
            }
            result[r] = sum;
        }
        result
    }
}

#[macro_export]
macro_rules! mat {
    ( $($x:expr), * $(,)?) => {
		{
			let mut v = Vec::new();
			$(
				v.push($x);
			)*
			Matrix::new(&v)
		}
	};
}

#[cfg(test)]
mod tests {
    use crate::{
        lib_test::{assert_almost_eq_mat, assert_almost_eq_tuple},
        point,
    };

    use super::*;

    #[test]
    fn create_4x4_matrix() {
        #[rustfmt::skip]
		let m: Matrix4 = mat![
			1.0, 2.0, 3.0, 4.0,
        	5.5, 6.5, 7.5, 8.5,
        	9.0, 10.0, 11.0, 12.0,
	      	13.5, 14.5, 15.5, 16.5,
		];

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn access_2x3_matrix() {
        #[rustfmt::skip]
		let m: Matrix<2, 3> = mat![
			1.0, 2.0, 3.0,
			4.0, 5.0, 6.0
		];

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(0, 2)], 3.0);
        assert_eq!(m[(1, 0)], 4.0);
        assert_eq!(m[(1, 1)], 5.0);
        assert_eq!(m[(1, 2)], 6.0);
    }

    #[test]
    fn matrix_equal() {
        let m1: Matrix<2, 2> = mat![1.0, 2.0, 3.0, 4.0];
        let m2: Matrix<2, 2> = mat![1.0, 2.0, 3.0, 4.0];

        assert_almost_eq_mat(m1, m2);
    }

    #[test]
    fn matrix_not_equal() {
        let m1: Matrix<2, 2> = mat![1.0, 2.0, 3.0, 4.0];
        let m2: Matrix<2, 2> = mat![4.0, 3.0, 2.0, 1.0];

        assert!(!Matrix::almost_eq(m1, m2));
    }

    #[test]
    fn mul_two_matrix_diff_size() {
        let m1: Matrix<2, 3> = mat![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let m2: Matrix<3, 2> = mat![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let expected: Matrix2 = mat![22.0, 28.0, 49.0, 64.0];
        assert_almost_eq_mat(m1 * m2, expected);
    }

    #[test]
    fn mul_two_matrices() {
        #[rustfmt::skip]
		let m1: Matrix4 = mat![
			1.0, 2.0, 3.0, 4.0,
			5.0, 6.0, 7.0, 8.0,
			9.0, 8.0, 7.0, 6.0,
			5.0, 4.0, 3.0, 2.0,
		];

        #[rustfmt::skip]
		let m2: Matrix4 = mat![
			-2.0, 1.0, 2.0, 3.0,
			3.0, 2.0, 1.0, -1.0,
			4.0, 3.0, 6.0, 5.0,
			1.0, 2.0, 7.0, 8.0,
		];

        #[rustfmt::skip]
		let result: Matrix4 = mat![
			20.0, 22.0, 50.0, 48.0,
			44.0, 54.0, 114.0, 108.0,
			40.0, 58.0, 110.0, 102.0,
			16.0, 26.0, 46.0, 42.0,
		];

        assert_almost_eq_mat(m1 * m2, result);
    }

    #[test]
    fn matrix_multiply_by_tuple() {
        #[rustfmt::skip]
		let m: Matrix4 = mat! [
			1.0, 2.0, 3.0, 4.0,
			2.0, 4.0, 4.0, 2.0,
			8.0, 6.0, 4.0, 1.0,
			0.0, 0.0, 0.0, 1.0,
		];

        let v = point(1.0, 2.0, 3.0);
        assert_almost_eq_tuple(m * v, point(18.0, 24.0, 33.0));
    }

    #[test]
    fn multiply_matrix_by_identity_matrix() {
        let m1: Matrix4 =
            mat![1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,];

        let m2: Matrix4 = Matrix::identity();

        assert!(m1 * m2 == m1);
    }

    #[test]
    fn transpose_matrix() {
        let m: Matrix<2, 3> = mat![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected: Matrix<3, 2> = mat![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];

        assert!(m.transpose() == expected);
    }

    #[test]
    fn submatrix() {
        #[rustfmt::skip]
		let m: Matrix3 = mat![
			1.0, 5.0, 0.0,
			-3.0, 2.0, 7.0,
			0.0, 6.0, -3.0,
		];

        #[rustfmt::skip]
		let expected: Matrix2 = mat![
			-3.0, 2.0,
			0.0, 6.0,
		];

        assert_almost_eq_mat(m.submatrix(0, 2), expected);

        #[rustfmt::skip]
		let m: Matrix4 = mat![
			-6.0, 1.0, 1.0, 6.0,
			-8.0, 5.0, 8.0, 6.0,
			-1.0, 0.0, 8.0, 2.0,
			-7.0, 1.0, -1.0, 1.0,
		];

        #[rustfmt::skip]
		let expected: Matrix3 = mat![
			-6.0, 1.0, 6.0,
			-8.0, 8.0, 6.0,
			-7.0, -1.0, 1.0,
		];

        assert_almost_eq_mat(m.submatrix(2, 1), expected);
    }

    #[test]
    fn minor_of_matrix3() {
        #[rustfmt::skip]
		let m: Matrix3 = mat![
			3.0, 5.0, 0.0,
			2.0, -1.0, -7.0,
			6.0, -1.0, 5.0,
		];

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_of_matrix3() {
        #[rustfmt::skip]
        let m: Matrix3 = mat![
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0,
        ];

        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn determinant_of_matrix4() {
        #[rustfmt::skip]
		let m: Matrix4 = mat![
			-2.0, -8.0, 3.0, 5.0,
			-3.0, 1.0, 7.0, 3.0,
			1.0, 2.0, -9.0, 6.0,
			-6.0, 7.0, 7.0, -9.0,
		];

        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn inverse_of_matrix4() {
        #[rustfmt::skip]
		let m: Matrix4 = mat![
			-5.0, 2.0, 6.0, -8.0,
			1.0, -5.0, 1.0, 8.0,
			7.0, 7.0, -6.0, -7.0,
			1.0, -3.0, 7.0, 4.0,
		];

        assert_eq!(m.determinant(), 532.0);
        assert_eq!(m.cofactor(2, 3), -160.0);
        assert_eq!(m.cofactor(3, 2), 105.0);

        #[rustfmt::skip]
		let expected: Matrix4 = mat![
			0.21805, 0.45113, 0.24060, -0.04511,
			-0.80827, -1.45677, -0.44361, 0.52068,
			-0.07895, -0.22368, -0.05263, 0.19737,
			-0.52256, -0.81391, -0.30075, 0.30639,
		];

        assert_almost_eq_mat(m.inverse().unwrap(), expected);
    }
}

use std::ops::{Index, IndexMut};

#[cfg(test)]
use crate::lib_test::almost_eq_f32;

#[derive(Copy, Clone, Debug)]
pub struct Matrix<const R: usize, const C: usize> {
    columns: [Column<R>; C],
}

pub type Matrix2 = Matrix<2, 2>;
pub type Matrix3 = Matrix<3, 3>;
pub type Matrix4 = Matrix<4, 4>;

#[derive(Copy, Clone, Debug)]
struct Column<const M: usize> {
    values: [f32; M],
}

impl<const M: usize> Default for Column<M> {
    fn default() -> Self {
        Self { values: [0.0; M] }
    }
}

impl<const M: usize> Index<usize> for Column<M> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<const M: usize> IndexMut<usize> for Column<M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
    fn default() -> Self {
        Self {
            columns: [Column::default(); C],
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
impl<const RC: usize> Matrix<RC, RC> {
    pub fn identity() -> Self {
        let mut m = Matrix::default();
        for i in 0..RC {
            m[(i, i)] = 1.0;
        }
        m
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    #[allow(dead_code)]
    fn assign(&mut self, values: &[f32]) {
        for r in 0..R {
            for c in 0..C {
                self.columns[c][r] = values[r * C + c];
            }
        }
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

#[macro_export]
macro_rules! mat {
    ( $($x:expr), * $(,)?) => {
		{
			let mut v = Vec::new();
			$(
				v.push($x);
			)*
			let mut m = Matrix::default();
			m.assign(&v);
			m
		}
	};
}

#[cfg(test)]
mod tests {
    use crate::lib_test::assert_almost_eq_mat;

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
}

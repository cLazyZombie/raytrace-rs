use crate::{Color, Matrix, V4};

pub(crate) fn assert_almost_eq_f32(lhs: f32, rhs: f32) {
    let eq = almost_eq_f32(lhs, rhs);
    assert!(eq, "{:?} it not equal to {:?}", lhs, rhs);
}

pub(crate) fn assert_almost_eq_v4(lhs: V4, rhs: V4) {
    let eq1 = almost_eq_f32(lhs.x, rhs.x);
    let eq2 = almost_eq_f32(lhs.y, rhs.y);
    let eq3 = almost_eq_f32(lhs.z, rhs.z);
    let eq4 = almost_eq_f32(lhs.w, rhs.w);
    assert!(
        eq1 && eq2 && eq3 && eq4,
        "{:?} is not equal to {:?}",
        lhs,
        rhs
    );
}

pub(crate) fn assert_almost_eq_color(lhs: Color, rhs: Color) {
    let eq1 = almost_eq_f32(lhs.red, rhs.red);
    let eq2 = almost_eq_f32(lhs.green, rhs.green);
    let eq3 = almost_eq_f32(lhs.blue, rhs.blue);
    assert!(eq1 && eq2 && eq3, "{:?} is not equal to {:?}", lhs, rhs);
}

pub(crate) fn assert_almost_eq_mat<const R: usize, const C: usize>(
    lhs: Matrix<R, C>,
    rhs: Matrix<R, C>,
) {
    let eq = Matrix::<R, C>::almost_eq(lhs, rhs);
    assert!(eq, "{:?} it not equal to {:?}", lhs, rhs);
}

pub(crate) fn almost_eq_f32(a: f32, b: f32) -> bool {
    let diff = f32::abs(a - b);
    diff <= f32::EPSILON
}

use crate::Vec4;

pub struct Sphere {
    pub center: Vec4,
    pub r: f32,
}

impl Sphere {
    pub fn new(center: Vec4, r: f32) -> Self {
        Self { center, r }
    }

    pub fn normal_at(&self, p: Vec4) -> Vec4 {
        let dir = p - self.center;
        dir.normalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::{lib_test::assert_almost_eq_tuple, point, vector};

    use super::*;

    #[test]
    fn normal_at_sphere() {
        let sphere = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        let sqrt_3_over_3 = f32::sqrt(3.0) / 3.0;
        let normal = sphere.normal_at(point(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3));
        assert_almost_eq_tuple(normal, vector(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3));
    }
}

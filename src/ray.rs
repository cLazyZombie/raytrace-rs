use crate::{Sphere, Vec4};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec4,
    pub dir: Vec4,
}

impl Ray {
    pub fn new(origin: Vec4, dir: Vec4) -> Self {
        Self { origin, dir }
    }

    pub fn position(&self, t: f32) -> Vec4 {
        self.dir * t + self.origin
    }

    pub fn intersect_sphere(&self, sphere: &Sphere) -> Vec<f32> {
        let oc = self.origin - sphere.center;
        let a = self.dir.dot(self.dir);
        let b = 2.0 * oc.dot(self.dir);
        let c = oc.dot(oc) - sphere.r * sphere.r;
        let det = b * b - 4.0 * a * c;

        let mut result = Vec::new();

        if det < 0.0 {
            result
        } else {
            // if det == 0.0 {
            //     result.push(-b / (2.0 * a));
            //     result
            // } else {
            let sqrt_det = det.sqrt();
            let t1 = (-b - sqrt_det) / (2.0 * a);
            let t2 = (-b + sqrt_det) / (2.0 * a);
            result.push(t1);
            result.push(t2);
            result
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lib_test::{assert_almost_eq_f32, assert_almost_eq_tuple},
        point, vector,
    };

    use super::*;

    #[test]
    fn create_ray() {
        let ray = Ray::new(point(1.0, 2.0, 3.0), vector(4.0, 5.0, 6.0));
        assert_eq!(ray.origin, point(1.0, 2.0, 3.0));
        assert_eq!(ray.dir, vector(4.0, 5.0, 6.0));
    }

    #[test]
    fn computing_point_from_distance() {
        let ray = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_almost_eq_tuple(ray.position(0.0), point(2.0, 3.0, 4.0));
        assert_almost_eq_tuple(ray.position(1.0), point(3.0, 3.0, 4.0));
        assert_almost_eq_tuple(ray.position(-1.0), point(1.0, 3.0, 4.0));
        assert_almost_eq_tuple(ray.position(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        let xs = ray.intersect_sphere(&sphere);
        assert_eq!(xs.len(), 2);
        assert_almost_eq_f32(xs[0], 4.0);
        assert_almost_eq_f32(xs[1], 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        let xs = ray.intersect_sphere(&sphere);
        assert_eq!(xs.len(), 2);
        assert_almost_eq_f32(xs[0], 5.0);
        assert_almost_eq_f32(xs[1], 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        let xs = ray.intersect_sphere(&sphere);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originages_inside_sphere() {
        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        let xs = ray.intersect_sphere(&sphere);
        assert_eq!(xs.len(), 2);
        assert_almost_eq_f32(xs[0], -1.0);
        assert_almost_eq_f32(xs[1], 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        let xs = ray.intersect_sphere(&sphere);
        assert_eq!(xs.len(), 2);
        assert_almost_eq_f32(xs[0], -6.0);
        assert_almost_eq_f32(xs[1], -4.0);
    }
}

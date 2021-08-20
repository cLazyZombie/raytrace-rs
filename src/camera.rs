use crate::{mat, point, vector, Angle, Matrix, Matrix4, Ray, Vec4};

pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub fov: Angle,             // vertical fov
    pub aspect: f32,            // w / h
    pub inv_transform: Matrix4, // inverser view transform
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, fov: Angle, aspect: f32, transform: Matrix4) -> Self {
        Self {
            hsize,
            vsize,
            fov,
            aspect,
            inv_transform: transform.inverse().unwrap(),
        }
    }

    pub fn dir(&self) -> Vec4 {
        self.inv_transform * vector(0.0, 0.0, 1.0)
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        // frustum의 크기를 구한다 (z가 1이라고 가정)
        let tan = f32::tan(self.fov.radian() / 2.0);
        let half_height = tan;
        let half_width = tan * self.aspect;

        // -1 to 1
        let fx = ((x as f32) / ((self.hsize - 1) as f32)) * 2.0 - 1.0;
        let fy = ((y as f32) / ((self.vsize - 1) as f32)) * -2.0 + 1.0;

        // create ray
        let ray_start = self.inv_transform * point(0.0, 0.0, 0.0);
        let local_ray_dir = vector(fx * half_width, fy * half_height, 1.0).normalize();
        let ray_dir = self.inv_transform * local_ray_dir;

        Ray::new(ray_start, ray_dir)
    }
}

pub fn view_transform(from: Vec4, to: Vec4, upv: Vec4) -> Matrix4 {
    let upv = upv.normalize();
    let dirv = (to - from).normalize();
    let rightv = upv.cross(dirv);
    let upv = dirv.cross(rightv);

    #[rustfmt::skip]
	let rot: Matrix4 = mat![
		rightv[0], rightv[1], rightv[2], 0.0,
		upv[0], upv[1], upv[2], 0.0,
		dirv[0], dirv[1], dirv[2], 0.0,
		0.0, 0.0, 0.0, 1.0,
	];

    let translate = Matrix4::translate(point(-from[0], -from[1], -from[2]));

    rot * translate
}

#[cfg(test)]
mod tests {
    use crate::{lib_test::assert_almost_eq_mat, point, vector};

    use super::*;

    #[test]
    fn identity_view_transform() {
        let view_transform = view_transform(point(0.0, 0.0, 0.0), point(0.0, 0.0, 1.0), vector(0.0, 1.0, 0.0));
        assert_almost_eq_mat(view_transform, Matrix4::identity());
    }

    #[test]
    fn view_transform_move_world() {
        let view_transform = view_transform(point(0.0, 0.0, 8.0), point(0.0, 0.0, 9.0), vector(0.0, 1.0, 0.0));
        assert_almost_eq_mat(view_transform, Matrix4::translate(point(0.0, 0.0, -8.0)));
    }
}

use crate::{vector, Intersection, Material, Object, Vec4};

pub struct Aabb {
    pub min: Vec4,
    pub max: Vec4,
    pub normal_outside: bool, // 상자 밖이 보이는 쪽인지 (false -> 안쪽에서 보이는 육면체)
    pub mat: Material,
}

impl Aabb {
    pub fn new(min: Vec4, max: Vec4, normal_outside: bool) -> Self {
        Self {
            min,
            max,
            normal_outside,
            mat: Material::default(),
        }
    }

    pub fn normal_at(&self, pos: Vec4) -> Vec4 {
        let normal = if nearly_close(self.min[0], pos[0]) {
            vector(-1.0, 0.0, 0.0)
        } else if nearly_close(self.max[0], pos[0]) {
            vector(1.0, 0.0, 0.0)
        } else if nearly_close(self.min[1], pos[1]) {
            vector(0.0, -1.0, 0.0)
        } else if nearly_close(self.max[1], pos[1]) {
            vector(0.0, 1.0, 0.0)
        } else if nearly_close(self.min[2], pos[2]) {
            vector(0.0, 0.0, -1.0)
        } else {
            vector(0.0, 0.0, 1.0)
        };

        if self.normal_outside {
            normal
        } else {
            -normal
        }
    }
}

fn nearly_close(a: f32, b: f32) -> bool {
    let diff = (a - b).abs();
    return diff < 0.1;
}

impl Object for Aabb {
    fn ray_intersect(&self, ray: &crate::Ray) -> Vec<crate::Intersection> {
        let ts = ray.intersect_aabb(self);
        ts.iter()
            .map(|t| {
                let pos = ray.position(*t);
                let normalv = self.normal_at(pos);
                Intersection::new(*t, pos, normalv, &self.mat)
            })
            .collect()
    }
}

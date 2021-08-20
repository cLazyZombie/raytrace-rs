use crate::{Intersection, Ray};

pub trait Object {
    fn ray_intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

use crate::{Material, Vec4};

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    pub pos: Vec4,
    pub normalv: Vec4,
    pub material: &'a Material,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, pos: Vec4, normalv: Vec4, material: &'a Material) -> Self {
        Self {
            t,
            pos,
            normalv,
            material,
        }
    }
}

pub fn get_frontmost_intersection<'a>(
    mut intersections: Vec<Intersection<'a>>,
) -> Option<Intersection<'a>> {
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    let first = intersections.iter().filter(|a| a.t > 0.0).next();

    first.map(|i| *i)
}

#[cfg(test)]
mod test {
    use crate::{point, vector};

    use super::*;

    #[test]
    fn front_most() {
        let mat = Material::default();
        let i1 = Intersection::new(1.0, point(1.0, 1.0, 1.0), vector(0.0, 1.0, 0.0), &mat);
        let i2 = Intersection::new(-1.0, point(1.0, 1.0, 1.0), vector(0.0, 1.0, 0.0), &mat);
        let i3 = Intersection::new(2.0, point(1.0, 1.0, 1.0), vector(0.0, 1.0, 0.0), &mat);

        let intersections = vec![i1, i2, i3];
        let front_most = get_frontmost_intersection(intersections).unwrap();
        assert_eq!(front_most.t, 1.0);
    }
}

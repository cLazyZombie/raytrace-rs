use crate::{
    point, point_lighting, Color, Intersection, Material, MaterialPattern, Object, PointLight, Ray, Sphere, Vec4,
};

pub struct World {
    pub objects: Vec<Box<dyn Object>>,
    pub point_lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            point_lights: Vec::new(),
        }
    }

    pub fn add_object<O: Object + 'static>(&mut self, obj: O) {
        self.objects.push(Box::new(obj));
    }

    pub fn add_pointlight(&mut self, light: PointLight) {
        self.point_lights.push(light);
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for obj in &self.objects {
            let mut cur_intersections = obj.ray_intersect(ray);
            intersections.append(&mut cur_intersections);
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    pub fn shade(&self, ray: &Ray, eyev: Vec4) -> Color {
        let intersections = self.intersect(ray);

        if intersections.len() == 0 {
            Color::BLACK
        } else {
            let front_most = intersections.iter().filter(|i| i.t > 0.0).next();
            if let Some(front_most) = front_most {
                //let mut acc_color = Color::BLACK;
                let mut acc_color = front_most.material.color(front_most.pos) * front_most.material.ambient;
                for light in &self.point_lights {
                    let is_shadowed = self.is_shadowed(front_most.pos, light);
                    acc_color += point_lighting(
                        front_most.material,
                        light,
                        front_most.pos,
                        eyev,
                        front_most.normalv,
                        is_shadowed,
                    );
                }
                acc_color
            } else {
                Color::BLACK
            }
        }
    }

    pub fn is_shadowed(&self, pos: Vec4, light: &PointLight) -> bool {
        let obj_to_light = light.pos - pos;
        let obj_to_light_v = obj_to_light.normalize();
        let ray = Ray::new(pos, obj_to_light_v);
        let intersections = self.intersect(&ray);

        if intersections.len() == 0 {
            false
        } else {
            if let Some(_) = intersections
                .iter()
                .filter(|i| i.t > 0.01 && i.t <= obj_to_light.mag() && i.normalv.dot(obj_to_light_v) > 0.0)
                .next()
            {
                true
            } else {
                false
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut obj1 = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        obj1.mat = Material::new(Color::new(0.8, 0.2, 0.6), MaterialPattern::Solid, 0.2, 0.7, 0.2, 200.0);

        let mut obj2 = Sphere::new(point(0.0, 0.0, 0.0), 0.5);
        obj2.mat = Material::new(Color::new(0.2, 0.6, 0.8), MaterialPattern::Solid, 0.2, 0.7, 0.2, 200.0);

        let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::WHITE);

        let mut world = World::new();
        world.add_object(obj1);
        world.add_object(obj2);
        world.add_pointlight(light);

        world
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lib_test::{assert_almost_eq_color, assert_almost_eq_f32},
        vector, Ray,
    };

    use super::*;

    #[test]
    fn create_world() {
        let world = World::new();
        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.point_lights.len(), 0);
    }

    #[test]
    fn default_world() {
        let world = World::default();
        assert_eq!(world.objects.len(), 2);
        assert_eq!(world.point_lights.len(), 1);
    }

    #[test]
    fn intersect_world_with_ray() {
        let world = World::default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let intersects = world.intersect(&ray);

        assert_eq!(intersects.len(), 4);
        assert_almost_eq_f32(intersects[0].t, 4.0);
        assert_almost_eq_f32(intersects[1].t, 4.5);
        assert_almost_eq_f32(intersects[2].t, 5.5);
        assert_almost_eq_f32(intersects[3].t, 6.0);
    }

    #[test]
    fn shade_world_with_ray() {
        let world = World::default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let color = world.shade(&ray, vector(0.0, 0.0, 1.0));
        assert_almost_eq_color(color, Color::new(0.46066123, 0.11516531, 0.34549594));
    }

    #[test]
    fn is_shadowed() {
        let world = World::default();
        let light = &world.point_lights[0]; // -10, 10, -10,
        assert_eq!(world.is_shadowed(point(10.0, -10.0, 10.0), light), true);
        assert_eq!(world.is_shadowed(point(-20.0, 20.0, -20.0), light), false);
        assert_eq!(world.is_shadowed(point(-2.0, 2.0, -2.0), light), false);
    }
}

use crate::{Color, Material, Vec4};

pub struct PointLight {
    pub pos: Vec4,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(pos: Vec4, intensity: Color) -> Self {
        Self { pos, intensity }
    }
}

pub fn point_lighting(material: &Material, light: &PointLight, position: Vec4, eyev: Vec4, normalv: Vec4) -> Color {
    let effective_color = material.color * light.intensity;
    let lightv = (light.pos - position).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = lightv.dot(normalv);

    let diffuse;
    let specular;
    if light_dot_normal < 0.0 {
        diffuse = Color::BLACK;
        specular = Color::BLACK;
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(-eyev);
        if reflect_dot_eye <= 0.0 {
            specular = Color::BLACK;
        } else {
            let factor = f32::powf(reflect_dot_eye, material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lib_test::assert_almost_eq_color, point, vector};

    #[test]
    fn lighting_with_the_eye_between_light_and_surface() {
        let eyev = vector(0.0, 0.0, 1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::WHITE);
        let material = Material::default();
        let result = point_lighting(&material, &light, position, eyev, normalv);
        assert_almost_eq_color(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
        let eyev = vector(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::WHITE);
        let material = Material::default();
        let result = point_lighting(&material, &light, position, eyev, normalv);
        assert_almost_eq_color(result, Color::new(1.0, 1.0, 1.0));
    }
}

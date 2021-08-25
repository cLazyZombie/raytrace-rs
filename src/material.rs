use crate::{Color, Vec4};

#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub pattern: MaterialPattern,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum MaterialPattern {
    Solid,
    Check,
}

impl Material {
    pub fn new(
        color: Color,
        pattern: MaterialPattern,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Self {
        Self {
            color,
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            pattern: MaterialPattern::Solid,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn color(&self, pos: Vec4) -> Color {
        match self.pattern {
            MaterialPattern::Solid => self.color,

            MaterialPattern::Check => {
                let x = is_check_dark(pos[0]);
                let y = is_check_dark(pos[1]);
                let z = is_check_dark(pos[2]);

                let count = [x, y, z].iter().filter(|v| **v).count();

                if count % 2 == 0 {
                    self.color
                } else {
                    self.color * 0.5
                }
            }
        }
    }
}

fn is_check_dark(v: f32) -> bool {
    let x = v / 0.47;
    let mut xi = (x.abs() as i32) % 2 == 0;
    if x.is_sign_negative() {
        xi = !xi;
    }

    xi
}

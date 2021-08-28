use crate::Color;

pub struct Canvas {
    width: u32,
    height: u32,
    buff: Vec<Color>,
    gamma: f32,
}

impl Canvas {
    /// gamme: gamma correction value. pow(color, 1/gamma). 2.2 if None
    pub fn new(width: u32, height: u32, gamma: Option<f32>) -> Self {
        let buff = vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize];
        let gamma = if let Some(gamma) = gamma { gamma } else { 2.2 };
        Self {
            width,
            height,
            buff,
            gamma,
        }
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        let idx = self.get_index(x, y);
        self.buff[idx as usize]
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        let idx = self.get_index(x, y);
        self.buff[idx as usize] = color
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn save_to_file(&self, path: &str) {
        let mut image = image::ImageBuffer::new(self.width, self.height);

        for (idx, color) in self.buff.iter().enumerate() {
            let (x, y) = Self::get_xy(self.width, idx as u32);

            let color = image::Rgb([
                (f32::powf(color.red.clamp(0.0, 1.0), self.gamma) * 255.0) as u8,
                (f32::powf(color.green.clamp(0.0, 1.0), self.gamma) * 255.0) as u8,
                (f32::powf(color.blue.clamp(0.0, 1.0), self.gamma) * 255.0) as u8,
            ]);

            *image.get_pixel_mut(x, y) = color;
        }

        image.save(path).unwrap();
    }

    fn get_index(&self, x: u32, y: u32) -> u32 {
        x + y * self.width
    }

    fn get_xy(width: u32, idx: u32) -> (u32, u32) {
        let x = idx % width;
        let y = idx / width;

        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib_test::assert_almost_eq_color;

    use super::*;

    #[test]
    fn create() {
        let canvas = Canvas::new(100, 100, None);
        for x in 0..100 {
            for y in 0..100 {
                let c = canvas.pixel_at(x, y);
                assert_almost_eq_color(c, Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20, None);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);
        assert_almost_eq_color(canvas.pixel_at(2, 3), red);
    }
}

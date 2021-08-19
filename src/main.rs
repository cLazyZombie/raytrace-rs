use raytrace_rs::{
    point, point_lighting, vector, Canvas, Color, Material, PointLight, Ray, Sphere,
};

fn main() {
    let eye = point(0.0, 0.0, 0.0);

    let light = PointLight::new(point(-5.0, 5.0, 5.0), Color::WHITE);

    let mut sphere = Sphere::new(point(0.0, 0.0, 10.0), 1.0);
    sphere.mat = Material::new(Color::new(1.0, 0.0, 0.0), 0.1, 0.7, 0.8, 200.0);

    let (width, height) = (1024, 1024);
    let mut canvas = Canvas::new(width, height);

    let near_z = 1.0;
    let near_width = 1.0;
    let near_height = 1.0;

    for ix in 0..width {
        for iy in 0..height {
            let x = ix as f32;
            let y = iy as f32;
            // calc ray

            // plane pos
            let x = ((x / ((width - 1) as f32)) - 0.5) * 2.0; // -1 to 1
            let y = ((y / ((height - 1) as f32)) - 0.5) * -2.0; // -1 to 1
            let dirv = vector(x * near_width * 0.5, y * near_height * 0.5, near_z).normalize();

            let ray = Ray::new(eye, dirv);

            let mut xs = ray.intersect_sphere(&sphere);
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

            // 가깝지만 뒤로 넘어가지 않는 위치를 구한다
            if xs.len() > 0 {
                let c = xs.iter().filter(|&&t| t > 0.0).next();
                if let Some(&t) = c {
                    let pos = ray.position(t);
                    let normalv = sphere.normal_at(pos);

                    let color = point_lighting(&sphere.mat, &light, pos, dirv, normalv);

                    canvas.write_pixel(ix, iy, color);
                }
            }
        }
    }

    canvas.save_to_file("output.png");
}

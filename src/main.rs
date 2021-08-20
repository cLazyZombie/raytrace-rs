use raytrace_rs::{
    get_frontmost_intersection, point, point_lighting, vector, Canvas, Color, Intersection, Material, PointLight, Ray,
    Sphere,
};

fn main() {
    let eye = point(0.0, 0.0, 0.0);
    let eyev = point(0.0, 0.0, 1.0);

    let light = PointLight::new(point(-5.0, 5.0, 5.0), Color::WHITE);

    let mut sphere = Sphere::new(point(0.0, 0.0, 10.0), 1.0);
    //sphere.mat = Material::new(Color::new(0.8, 0.4, 0.2), 0.1, 0.7, 10.0, 200.0);
    sphere.mat = Material::new(Color::new(0.8, 0.4, 0.2), 0.2, 0.8, 1.0, 100.0);

    let (width, height) = (1024, 1024);
    let mut canvas = Canvas::new(width, height);

    let near_z = 1.0;
    let near_width = 0.4;
    let near_height = 0.4;

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

            // ray와 hit되는 intersection들을 구한다
            let intersections: Vec<_> = ray
                .intersect_sphere(&sphere)
                .iter()
                .map(|t| {
                    let pos = ray.position(*t);
                    let normalv = sphere.normal_at(pos);
                    Intersection::new(*t, pos, normalv, &sphere.mat)
                })
                .collect();

            // 가장 앞에 있는 hit를 구한다
            let frontmost = get_frontmost_intersection(intersections);
            if let Some(frontmost) = frontmost {
                let color = point_lighting(frontmost.material, &light, frontmost.pos, eyev, frontmost.normalv);
                canvas.write_pixel(ix, iy, color);
            }
        }
    }

    canvas.save_to_file("output.png");
}

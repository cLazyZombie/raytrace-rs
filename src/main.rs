use raytrace_rs::{point, vector, Canvas, Color, Material, PointLight, Ray, Sphere, World};

fn main() {
    let eye = point(0.0, 0.0, 0.0);

    let mut world = World::new();

    let light1 = PointLight::new(point(-5.0, 5.0, 5.0), Color::WHITE);
    world.add_pointlight(light1);

    let light2 = PointLight::new(point(5.0, -5.0, 5.0), Color::new(1.0, 0.2, 0.4));
    world.add_pointlight(light2);

    let mut sphere1 = Sphere::new(point(-0.5, 0.0, 10.0), 1.0);
    sphere1.mat = Material::new(Color::new(0.8, 0.4, 0.2), 0.2, 0.8, 1.0, 200.0);
    world.add_object(sphere1);

    let mut sphere2 = Sphere::new(point(1.2, 0.0, 13.0), 0.7);
    sphere2.mat = Material::new(Color::new(0.2, 0.8, 0.4), 0.1, 0.6, 1.0, 200.0);
    world.add_object(sphere2);

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

            let color = world.shade(&ray);
            canvas.write_pixel(ix, iy, color);
        }
    }

    canvas.save_to_file("output.png");
}

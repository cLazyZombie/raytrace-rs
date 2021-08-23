use raytrace_rs::{
    point, vector, view_transform, Aabb, Angle, Camera, Canvas, Color, Material, PointLight, Sphere, World,
};

fn main() {
    let (width, height): (u32, u32) = (1600, 900);

    let eye = point(0.0, 3.0, 7.0);
    let target = point(0.0, 0.0, 12.0);
    let up = vector(0.0, 1.0, 0.0);
    let view_mat = view_transform(eye, target, up);
    let camera = Camera::new(
        width,
        height,
        Angle::from_degree(60.0),
        (width as f32) / (height as f32),
        view_mat,
    );

    let mut world = World::new();

    let light1 = PointLight::new(point(-5.0, 5.0, 5.0), Color::WHITE);
    world.add_pointlight(light1);

    let light2 = PointLight::new(point(5.0, -5.0, 5.0), Color::WHITE);
    world.add_pointlight(light2);

    let mut sphere1 = Sphere::new(point(-0.5, 0.0, 10.0), 1.0);
    sphere1.mat = Material::new(Color::new(0.8, 0.4, 0.2), 0.2, 0.8, 1.0, 200.0);
    world.add_object(sphere1);

    let mut sphere2 = Sphere::new(point(1.2, 0.0, 13.0), 0.7);
    sphere2.mat = Material::new(Color::new(0.2, 0.8, 0.4), 0.1, 0.6, 1.0, 200.0);
    world.add_object(sphere2);

    let mut aabb1 = Aabb::new(point(-15.0, -15.0, -15.0), point(15.0, 15.0, 25.0), false);
    aabb1.mat = Material::new(Color::new(0.3, 0.3, 0.3), 0.1, 0.4, 0.3, 100.0);
    world.add_object(aabb1);

    let mut aabb2 = Aabb::new(point(-1.0, -1.0, 15.0), point(1.0, 1.0, 16.0), true);
    aabb2.mat = Material::new(Color::new(1.0, 1.0, 1.0), 0.2, 0.3, 1.0, 100.0);
    world.add_object(aabb2);

    let mut canvas = Canvas::new(width, height);

    for ix in 0..width {
        for iy in 0..height {
            let ray = camera.get_ray(ix, iy);
            let color = world.shade(&ray, camera.dir());
            canvas.write_pixel(ix, iy, color);
        }
    }

    canvas.save_to_file("output.png");
}

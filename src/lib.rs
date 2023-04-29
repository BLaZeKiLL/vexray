use crate::{utils::vec3::{Vec3, Point3}, renderer::ray::{Ray, ray_color}, world::World, shapes::sphere::Sphere};

mod image;
mod utils;
mod world;
mod shapes;
mod renderer;

pub fn run(file: &str, width: u32, height: u32) {
    println!("Welcome to VexRay");

	let world = &mut World::new();

	let sphere1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
	let sphere2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);

	world.add(&sphere1);
	world.add(&sphere2);

    // Image setup
    let w = width as f64;
    let h = height as f64;

    let aspect_ratio = w / h;

    // Camera setup
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin 
        - horizontal / 2.0 
        - vertical / 2.0 
        - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut buffer = String::new();

    for row in (0..height).rev() {
        println!("Progress : {:.2}%", ((height - row) as f64 / h) * 100.0);
        for col in 0..width {
            let u = col as f64 / w;
            let v = row as f64 / h;
            
            let ray = Ray::new(
                origin, 
                lower_left_corner + u * horizontal + v * vertical - origin
            );

            let color = ray.color(world);

            buffer.push_str(&color.write_color());
        }
    }

    image::ppm_save_image(file, &buffer, width, height);
}

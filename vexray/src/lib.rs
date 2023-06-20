use crate::{utils::{vec3::{Point3, Color3}, math::Math}, renderer::camera::Camera, world::World, shapes::sphere::Sphere};

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
	let camera = Camera::new(aspect_ratio);

    // Output
	let samples = 128;
	let bounces = 64;

    // let mut buffer = String::new();

    // for row in (0..height).rev() {
    //     // println!("Progress : {:.2}%", ((height - row) as f64 / h) * 100.0);
    //     for col in 0..width {
    //         let mut color = Color3::black(); 

	// 		for _ in 0..samples {
	// 			let u = ((col as f64) + Math::random_double()) / w;
	// 			let v = ((row as f64) + Math::random_double()) / h;

	// 			let ray = camera.get_ray(u, v);

	// 			color += ray.color(world, bounces);
	// 		}

    //         buffer.push_str(&color.write_color(samples));
    //     }
    // }

	let buffer: String = (0..height).rev()
		.flat_map(|row| (0..width).map(move |col| (row, col)))
		.map(|(row, col)| -> String {
            let mut color = Color3::black(); 

			for _ in 0..samples {
				let u = ((col as f64) + Math::random_double()) / w;
				let v = ((row as f64) + Math::random_double()) / h;

				let ray = camera.get_ray(u, v);

				color += ray.color(world, bounces);
			}

			color.write_color(samples)
		})
		.collect();
		

    image::ppm_save_image(file, &buffer, width, height);
}
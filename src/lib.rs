use crate::utils::math::Color3;

mod image;
mod utils;
mod renderer;

pub fn run(file: &str, width: u32, height: u32) {
    println!("Welcome to VexRay");

    let mut buffer = String::new();

    let w = width as f64;
    let h = height as f64;

    for row in (0..height).rev() {
        println!("Progress : {:.2}%", ((height - row) as f64 / h) * 100.0);
        for col in 0..width {
            let r = col as f64 / w;
            let g = row as f64 / h;
            let b = 0.25;

            let color = Color3::new(r, g, b);

            buffer.push_str(&color.write_color());
        }
    }

    image::ppm_save_image(file, &buffer, width, height);
}

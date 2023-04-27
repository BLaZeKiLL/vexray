use std::fs;

pub fn ppm_save_image(file: &str, data: &str, width: u32, height: u32) {
  // Format of PPM file
  let mut buffer = format!("P3\n{} {}\n255\n", width, height);

  // Append data
  buffer.push_str(data);

  // Write file
  fs::write(file, buffer).expect("Unable to write file");

  println!("Image saved at {file}");
}
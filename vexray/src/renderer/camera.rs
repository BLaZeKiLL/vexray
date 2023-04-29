use crate::utils::vec3::{Point3, Vec3};

use super::ray::Ray;

pub struct Camera {
	origin: Point3,
	horizontal: Vec3,
	vertical: Vec3,
	lower_left_corner: Point3
}

impl Camera {
	pub fn new(aspect_ratio: f64) -> Camera {
		let height = 2.0;
		let focal_length = 1.0;

		let origin = Point3::zero();
		let horizontal = Vec3::new(aspect_ratio * height, 0.0, 0.0);
		let vertical = Vec3::new(0.0, height, 0.0);

		let lower_left_corner = origin 
        	- horizontal / 2.0 
        	- vertical / 2.0 
        	- Vec3::new(0.0, 0.0, focal_length);

		Camera { 
			origin, 
			horizontal, 
			vertical, 
			lower_left_corner
		}
	}

	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		Ray::new(
			self.origin, 
			self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
		)
	}

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn horizontal(&self) -> Vec3 {
        self.horizontal
    }

    pub fn vertical(&self) -> Vec3 {
        self.vertical
    }

    pub fn lower_left_corner(&self) -> Vec3 {
        self.lower_left_corner
    }
}
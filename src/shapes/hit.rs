use crate::{utils::math::{Point3, Vec3}, renderer::ray::Ray};

pub struct Hit {
	point: Point3,
	normal: Vec3,
	t: f64
}

impl Hit {
	pub fn new(point: Point3, normal: Vec3, t: f64) -> Hit {
		Hit { point, normal, t }
	}

	pub fn point(&self) -> Point3 {
		self.point
	}

	pub fn normal(&self) -> Vec3 {
		self.normal
	}

	pub fn t(&self) -> f64 {
		self.t
	}
}

pub enum HitResult {
	Fail,
	Success(Hit)
}

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult;
}
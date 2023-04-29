use crate::{utils::vec3::{Point3, Vec3}, renderer::ray::Ray};

pub struct Hit {
	point: Point3,
	normal: Vec3,
	t: f64,
	front_face: bool,
}

impl Hit {
	pub fn new(ray: &Ray, point: Point3, outward_normal: Vec3, t: f64) -> Hit {
		let front_face = Vec3::dot(&ray.direction(), &outward_normal) < 0.0;

		Hit { point, normal: if front_face {outward_normal} else {-outward_normal}, t, front_face }
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
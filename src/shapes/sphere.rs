use crate::{renderer::ray::Ray, utils::math::{Point3, Vec3}};

/// Solves the sphere equation, which is a quadratic equation
pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
	let oc = ray.origin() - center;
	
	let a = ray.direction().sqr_magnitude();
	let half_b = Vec3::dot(&oc, &ray.direction());
	let c = oc.sqr_magnitude() - radius * radius;

	let discriminant = half_b * half_b - a * c;

	if discriminant < 0.0 {
		-1.0
	} else {
		(-half_b - f64::sqrt(discriminant)) / a
	}
}

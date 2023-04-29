use crate::{
    renderer::ray::Ray,
    utils::vec3::{Point3, Vec3},
};

use super::hit::{HitResult, Hittable, Hit};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
	/// Solves the sphere equation, which is a quadratic equation
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult {
        let oc = ray.origin() - self.center;

        let a = ray.direction().sqr_magnitude();
        let half_b = Vec3::dot(&oc, &ray.direction());
        let c = oc.sqr_magnitude() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return HitResult::Fail;
        }

        let sqrtd = f64::sqrt(discriminant);

        let r1 = (-half_b - sqrtd) / a;

        let root = if r1 < t_min || r1 > t_max {
            let r2 = (-half_b + sqrtd) / a;

            if r2 < t_min || r2 > t_max {
                return HitResult::Fail;
            }

            r2
        } else {
            r1
        };

		let point = ray.at(root);

        HitResult::Success(Hit::new(ray, point, (point - self.center) / self.radius, root))
    }
}

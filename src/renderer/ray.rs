use crate::{utils::math::{Point3, Vec3, Color3}, shapes::sphere::hit_sphere};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
}

pub fn ray_color(ray: &Ray) -> Color3 {
	let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);

	if t > 0.0 {
		let normal = (ray.at(t) - Point3::new(0.0, 0.0, -1.0)).normalized();
		return 0.5 * (normal + Vec3::one());
	}

    let direction = ray.direction().normalized();
    let t = 0.5 * (direction.y() + 1.0);
	(1.0 - t) * Color3::white() + t * Color3::new(0.5, 0.7, 1.0)
}

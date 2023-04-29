use crate::{utils::vec3::{Point3, Vec3, Color3}, shapes::{sphere::Sphere, hit::{Hittable, HitResult}}, world::World};

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

	pub fn color(&self, world: &World) -> Color3 {
		let hit_result: HitResult = world.hit(self, 0.0, f64::MAX);

		match hit_result {
			HitResult::Fail => { // Background
				let direction = self.direction().normalized();
				let blend = 0.5 * (direction.y() + 1.0);
				(1.0 - blend) * Color3::white() + blend * Color3::new(0.5, 0.7, 1.0)
			},
			HitResult::Success(hit) => { // Normal of what we hit
				0.5 * (hit.normal() + Vec3::one())
			},
		}
	}
}

pub fn ray_color(ray: &Ray) -> Color3 {
	let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

	let hit_result = sphere.hit(ray, 0.0, f64::MAX);

	match hit_result {
		HitResult::Fail => { // Background
			let direction = ray.direction().normalized();
			let blend = 0.5 * (direction.y() + 1.0);
			(1.0 - blend) * Color3::white() + blend * Color3::new(0.5, 0.7, 1.0)
		},
		HitResult::Success(hit) => { // Normal of what we hit
			0.5 * (hit.normal() + Vec3::one())
		},
	}
}

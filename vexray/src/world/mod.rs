use crate::{shapes::hit::{Hittable, HitResult}, renderer::ray::Ray};

// I have no idea of how lifetimes work...

pub struct World<'a> {
	objects: Vec<&'a dyn Hittable>
}

impl <'a> World<'a> {
	pub fn new() -> World<'a> {
		World { objects: vec![] }
	}

	pub fn add(&mut self, hittable: &'a dyn Hittable) {
		self.objects.push(hittable);
	}
}

impl Hittable for World<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult {
		let mut closest_so_far = t_max;
		let mut hit_result = HitResult::Fail;

		for object in self.objects.iter() {
			let object_hit_result = object.hit(ray, t_min, closest_so_far);

			if let HitResult::Success(hit) = object_hit_result {
				closest_so_far = hit.t();
				hit_result = HitResult::Success(hit);
			}
		}

		hit_result
    }
}
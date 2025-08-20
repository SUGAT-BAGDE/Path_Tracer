use glam::Vec3;

use crate::Ray;

#[derive(Default)]
pub struct IntersectionPaylaod {
    t: f32, // intersection distance
    positiion: Vec3,
    normal: Vec3,
    material_id: i32,
}

trait Geometry {
    fn intersect_ray(&self, ray :Ray) -> IntersectionPaylaod;
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material_id: i32,
}

impl Geometry for Sphere {
    fn intersect_ray(&self, ray :Ray) -> IntersectionPaylaod {
        let origin = ray.origin - self.position;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(origin);
        let c = origin.dot(origin) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            // continue;
            todo!()
        }

        let sqrt_d = discriminant.sqrt();

        let _closest_t = (-b - sqrt_d) / (2.0 * a);
        IntersectionPaylaod {
            ..Default::default()
        }
    }
}

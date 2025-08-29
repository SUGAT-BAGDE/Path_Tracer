use glam::Vec3;

use crate::Ray;

#[derive(Default, Debug)]
pub struct HitPayload {
    pub hit_distance: f32,
    pub world_position: Vec3,
    pub world_normal: Vec3,

    pub object_index: Option<usize>,
    pub material_index: Option<usize>,
}

pub trait Geometry {
    fn intersect_ray(&self, ray :&Ray) -> Option<HitPayload>;
}

pub mod sphere;
pub use sphere::Sphere;

pub mod plane;
pub use plane::Plane;

pub mod triangle;
pub use triangle::Triangle;

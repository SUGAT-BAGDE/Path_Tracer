use glam::Vec3;

use crate::Ray;
use crate::geometry::{Geometry, HitPayload};

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,

    normal: Vec3,
    pub material_id : i32,
}

impl Triangle {
    pub fn new((v0, v1, v2):(Vec3, Vec3, Vec3), matrial: i32) -> Self {
        let normal = (v1 - v0).cross(v2 - v0).normalize();
        Self { v0, v1, v2, normal, material_id : matrial}
    }
}

// this is using egde cross the vector from test vertex to hit point
impl Geometry for Triangle {
    fn intersect_ray(&self, ray :&Ray) -> Option<HitPayload> {
        let normal = self.normal;
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < f32::EPSILON {
            return None;
        }

        let num = normal.dot(self.v0 - ray.origin);
        let t = num / denom;

        let facing_normal = if denom < 0.0 {
            normal
        }
        else {
            -normal
        };

        let phit = ray.origin + t * ray.direction;

        // test with respect with v0
        let egde_v0v1 = self.v1 - self.v0;
        let egde_v0phit = phit - self.v0;
        let dir_corresponding_to_v0 = egde_v0v1.cross(egde_v0phit);

        if self.normal.dot(dir_corresponding_to_v0) < 0.0 {
            return None;
        }
        
        // test with respect with v1
        let egde_v1v2 = self.v2 - self.v1;
        let egde_v1phit = phit - self.v1;
        let dir_corresponding_to_v1 = egde_v1v2.cross(egde_v1phit);

        if self.normal.dot(dir_corresponding_to_v1) < 0.0 {
            return None;
        }

        // test with respect with v1
        let egde_v2v0 = self.v0 - self.v2;
        let egde_v2phit = phit - self.v2;
        let dir_corresponding_to_v2 = egde_v2v0.cross(egde_v2phit);

        if self.normal.dot(dir_corresponding_to_v2) < 0.0 {
            return None;
        }

        // TODO: Barycentric coorinate

        let material = if self.material_id < 0 {
            None
        }
        else {
            Some(self.material_id as usize)
        };


        Some(
            HitPayload {
                hit_distance: t,
                world_position : phit,
                world_normal: facing_normal,
                material_index: material,
                ..Default::default()
            }
        )
    }
}

// pub struct AreaTestTriangle(Triangle);

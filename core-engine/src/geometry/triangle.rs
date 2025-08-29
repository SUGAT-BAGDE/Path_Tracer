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

        let edges = [
            (&self.v0, &self.v1, &self.v2), // for alpha
            (&self.v1, &self.v2, &self.v0), // for beta
            (&self.v2, &self.v0, &self.v1), // for gamma
        ];

        let mut barycentric_coords = [0.0; 3];

        for (i, (v_from, v_to, v_other)) in edges.into_iter().enumerate() {
            let edge = v_to - v_from;
            let to_phit = phit - v_from;
            let cross = edge.cross(to_phit);

            if self.normal.dot(cross) < 0.0 {
                return None;
            }

            let full_area = edge.cross(v_other - v_from).length();
            barycentric_coords[i] = cross.length() / full_area;
        }

        let _alpha = barycentric_coords[0];
        let _beta  = barycentric_coords[1];
        let _gamma  = barycentric_coords[2];

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

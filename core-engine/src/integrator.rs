use glam::{Vec3, Vec4};

use crate::Ray;
use crate::cameras::SharedCamera;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::materials::Matrial;

use crate::geometry::{Geometry, HitPayload, Plane, Triangle};

static DEFAULT_MATERIAL: Matrial = Matrial {
    albedo: Vec3::ONE,
    roughness: 0.5,
    metalic: 0.0,
    emission_color: Vec3::ZERO,
    emissive_power: 0.0,
};

#[derive(Clone, Copy)]
pub struct Integrator {
    pub bounces: usize,
    pub max_compulsory_bounces: usize,
}

impl Integrator {
    pub fn compute_incomming_radience(
        &mut self,
        scene: &Scene,
        x: u32,
        y: u32,
        camera: &SharedCamera,
        sampler: &mut Sampler,
    ) -> Vec4 /* returns radiance per RGB channel */ {
        let cam = camera.read().unwrap();
        let mut ray = cam.get_ray(x, y);
        drop(cam);

        let mut light = Vec3::ZERO;

        let mut contribution = Vec3::ONE;
        for bounce in 0..self.bounces {
            let payload = self.trace_ray(&ray, scene);

            if payload.hit_distance > 0.0 {
                let material = match payload.material_index {
                    Some(index) => {
                        scene
                            .materials
                            .get(index)
                            .unwrap_or(&DEFAULT_MATERIAL)
                    },
                    None => &DEFAULT_MATERIAL
                };

                light += material.emission_color * material.emissive_power * contribution;

                let normal = payload.world_normal;
                let wi = sampler.sample_hemisphere_cosine_weighted(normal);
                let cos_theta = wi.dot(normal).max(0.0);
                let pdf = cos_theta / std::f32::consts::PI;
                let brdf = material.albedo / std::f32::consts::PI;

                contribution *= brdf * cos_theta / pdf;

                if bounce >= self.max_compulsory_bounces {
                    let p = contribution.x.max(contribution.y.max(contribution.z));
                    if sampler.next_f32() > p {
                        break;
                    }
                    contribution /= p;
                }

                ray.origin = payload.world_position + payload.world_normal * f32::EPSILON;
                ray.direction = wi;
            } else {
                // sky box, or something
                let sky_color = match &scene.skybox {
                    Some(exr) => exr.sample(ray.direction),
                    None => scene.default_sky_color,
                };
                light += sky_color * contribution;
                break;
            }
        }
        Vec4::from((light, 1.0))
    }

    fn trace_ray(&self, ray: &Ray, scene: &Scene) -> HitPayload {
        // (bx^2 + by^2 + bz^2)t^2 + 2(axbx + ayby + azbz)t + (ax^2 + ay^2 + az^2 - r^2)
        // a vec ray origin
        // b vec ray direction
        // r radius
        // t hit distance

        let mut hit_distance = f32::MAX;
        let mut closest_hit = HitPayload {
            hit_distance: -1.0,
            ..Default::default()
        };

        for (_i, sphere) in scene.spheres.iter().enumerate() {
            if let Some(payload) = sphere.intersect_ray(ray) {
                if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                    hit_distance = payload.hit_distance;
                    closest_hit = payload;
                }
            }
            else {
                continue;
            }
        }

        // test code for plane
        let test_plane = Plane {
            position: -Vec3::Y,
            normal: Vec3::Y,
            material_id: 1
        };
        if let Some(payload) = test_plane.intersect_ray(ray) {
            if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                hit_distance = payload.hit_distance;
                closest_hit = payload;
            }
        }

        let test_triangle = Triangle::new((
        Vec3::new(2.0, 1.0, -2.0),
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 2.0, -2.0),
        ), 0);

        if let Some(payload) = test_triangle.intersect_ray(ray) {
            if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                closest_hit = payload;
            }
        }

        return closest_hit;
    }
}

use glam::{Vec3, Vec4};

use crate::cameras::{SharedCamera};
use crate::sampler::Sampler;
use crate::scene::{Matrial, Scene};
use crate::Ray;

static DEFAULT_MATERIAL : Matrial = Matrial {
    albedo : Vec3::ONE,
    roughness : 0.5,
    metalic : 0.0,
    emission_color : Vec3::ZERO,
    emissive_power : 0.0
};

pub struct Integrator {
    pub bounces : usize,
    pub max_compulsory_bounces : usize
}

#[derive(Default, Debug)]
struct HitPayload {
    hit_distance : f32,
    world_position : Vec3,
    world_normal : Vec3,

    object_index : Option<usize>,
}

impl Integrator {
        
    pub fn compute_incomming_radience(
        &mut self,
        scene: &Scene,
        x : u32,
        y : u32,
        camera : &SharedCamera, 
        sampler : &mut Sampler
    ) -> Vec4 /* returns color */  {

        let cam = camera.read().unwrap();
        let mut ray = cam.get_ray(x, y);
        drop(cam);

        let mut light = Vec3::ZERO;

        let mut contribution = Vec3::ONE;
        for bounce in 0..self.bounces {

            let payload = self.trace_ray(&ray, scene);

            if let Some(i) = payload.object_index {

                let sphere = scene.spheres.get(i).unwrap();

                let material = if sphere.material_id < 0 {
                    &DEFAULT_MATERIAL
                } else {
                    scene.materials
                        .get(sphere.material_id as usize)
                        .unwrap_or(&DEFAULT_MATERIAL)
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
            }
            else {
                // sky box, or something
                let sky_color = match &scene.skybox {
                    Some(exr) => exr.sample(ray.direction),
                    None => scene.default_sky_color
                };
                light += sky_color * contribution;
                break;
            }
        }
        Vec4::from((light, 1.0))
    }

    fn trace_ray(&self, ray : &Ray, scene: &Scene) -> HitPayload {
        // (bx^2 + by^2 + bz^2)t^2 + 2(axbx + ayby + azbz)t + (ax^2 + ay^2 + az^2 - r^2)
        // a vec ray origin
        // b vec ray direction
        // r radius
        // t hit distance

        let mut hit_distance = f32::MAX;
        // let mut closest_sphere : Option<&Sphere> = None;
        let mut closest_sphere : Option<usize> = None;

        for (i, sphere) in scene.spheres.iter().enumerate() {

            let origin = ray.origin - sphere.position;

            let a = ray.direction.dot(ray.direction);
            let b = 2.0 * ray.direction.dot(origin);
            let c = origin.dot(origin) - sphere.radius * sphere.radius;

            let discriminant = b * b - 4.0 * a * c;

            if discriminant < 0.0 {
                continue;
            }

            let sqrt_d = discriminant.sqrt();

            let closest_t = (-b - sqrt_d) / (2.0 * a);

            if closest_t > 0.0 && closest_t < hit_distance {
                closest_sphere = Some(i);
                hit_distance = closest_t;
            }
        }

        if let Some(sphere_index) = closest_sphere {
            self.closest_hit(scene, ray, hit_distance, sphere_index)
        }       
        else
        {
            self.ray_miss(ray)
        }
    }

    fn closest_hit(
        &self, 
        scene : &Scene, 
        ray : &Ray, 
        hit_distance : f32,
        object_index: usize
    ) -> HitPayload {
        let closest_sphere = scene.spheres.get(object_index).unwrap();

        let origin = ray.origin - closest_sphere.position;
        let hit_point = origin + hit_distance * ray.direction;

        let normal = hit_point.normalize();


        HitPayload {
            hit_distance: hit_distance,
            world_position: hit_point + closest_sphere.position,
            world_normal: normal,
            object_index : Some(object_index)
        }
    }

    fn ray_miss(&self, _ray : &Ray) -> HitPayload {
        HitPayload { 
            hit_distance: -1.0,
            ..Default::default()
        }
    }
}

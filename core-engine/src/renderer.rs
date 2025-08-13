use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use glam::{usize, Vec3, Vec4};

use crate::camera::PinholeCamera;
use crate::camera::Camera;
use crate::sampler::Sampler;
use crate::scene::{Matrial, Scene};
use crate::Ray;
use crate::utils::convert_to_argb;

static DEFAULT_MATERIAL : Matrial = Matrial {
    albedo : Vec3::ONE,
    roughness : 0.5,
    metalic : 0.0
};

pub struct RayTracer {
    width: u32,
    height: u32,
    frame_buffer : Vec<u32>,
    last_render_time: Duration,

    pub active_camera: Arc<RwLock<dyn Camera>>,
    // pub scene : Arc<Scene>

    sampler : Sampler,
}

#[derive(Default, Debug)]
struct HitPayload {
    hit_distance : f32,
    world_position : Vec3,
    world_normal : Vec3,

    object_index : Option<usize>,
}

impl RayTracer {
    pub fn new() -> Self {
        let camera =  PinholeCamera::new(
                Vec3::new(0.0, 0.0, 2.0), 
                Vec3::ZERO,
                35.0,
                55.0,
                [0,0]
            );

        Self {
            width: 0,
            height: 0,
            frame_buffer: vec![],
            active_camera : Arc::new(RwLock::new(camera)),
            last_render_time: Duration::from_secs(0),

            sampler : Sampler::new(),
        }
    }

    pub fn set_active_camera(&mut self, camera : Arc<RwLock<dyn Camera>>) {
        self.active_camera = camera;
    }

    pub fn get_current_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    pub fn prepare_pixels(&mut self, scene: &Scene, width: u32, height: u32) {
        // if self.pixels.len() == 0 {
        if self.frame_buffer.len() != (width * height) as usize
            || self.width != width
            || self.height != height
        {
            self.render(scene, width, height);
        }
    }

    fn set_size(&mut self, size: [u32; 2]) {
        self.width = size[0];
        self.height = size[1];
        let mut cam = self.active_camera.write().unwrap();
        cam.set_image_resolutions(size);
    }

    pub fn render(&mut self, scene: &Scene, width: u32, height: u32) {
        let render_start_time = Instant::now();

        self.set_size([width, height]);

        self.frame_buffer = vec![0xFFFFFFFF_u32; (width * height) as usize];
        // let aspect_ratio = width as f32 / height as f32;

        for y in 0..height {
            for x in 0..width {

                let color = convert_to_argb(
                    &self.per_pixel(scene, x, y)
                        .clamp(
                            Vec4::from((0.0, 0.0, 0.0, 0.0)),
                            Vec4::from((1.0, 1.0, 1.0, 1.0))
                        ),
                );

                self.frame_buffer[(y * width + x) as usize] = color;
            }
        }

        self.last_render_time = render_start_time.elapsed();
    }

    fn per_pixel(&mut self, scene: &Scene, x : u32, y : u32) -> Vec4 /* returns color */  {
        let cam = self.active_camera.read().unwrap();
        let mut ray = cam.get_ray(x, y);
        drop(cam);

        let mut color = Vec3::ZERO;
        let bounces = 5;

        let mut mutliplier = 1.0;
        for _ in 0..bounces {

            let payload = self.trace_ray(&ray, scene);

            if let Some(i) = payload.object_index {

                let light_origin = Vec3::new(-2.0, 1.0, 2.0);
                let light_direction = (Vec3::ZERO - light_origin).normalize();
                // let light_direction =Vec3::from((-1.0, -1.0, -1.0)).normalize();

                let radiance = payload.world_normal.dot(-light_direction).max(0.0);
                let sphere = scene.spheres.get(i).unwrap();

                let material = if sphere.material_id < 0 {
                    &DEFAULT_MATERIAL
                } else {
                    scene.materials.get(i).unwrap()
                }; 

                color += material.albedo * radiance * mutliplier;

                ray.origin = payload.world_position + payload.world_normal * f32::EPSILON;
                ray.direction = ray.direction.reflect(
                    (payload.world_normal + material.roughness * self.sampler.next_3d())
                        .normalize()
                );
            }
            else {
                // sky box, or something
                color += scene.sky_color * mutliplier;
                break;
            }
            mutliplier *= 0.5;
        }
        Vec4::from((color, 1.0))
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

    fn closest_hit(&self, 
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

    pub fn get_output(&self) -> &[u32] {
        self.frame_buffer.as_slice()
    }

    pub fn get_last_render_time(&self) -> Duration {
        self.last_render_time
    }
}

impl Default for RayTracer {
    fn default() -> Self {
        Self::new()
    }
}


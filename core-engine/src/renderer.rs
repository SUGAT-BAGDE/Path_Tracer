use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use glam::{Vec3, Vec4};

use crate::camera::PinholeCamera;
use crate::camera::Camera;
use crate::scene::{Scene, Sphere};
use crate::Ray;
use crate::utils::convert_to_argb;

pub struct RayTracer {
    width: u32,
    height: u32,
    frame_buffer : Vec<u32>,
    last_render_time: Duration,

    pub active_camera: Arc<RwLock<dyn Camera>>,
}

impl RayTracer {
    pub fn new() -> Self {
        let camera =  PinholeCamera::new(
                Vec3::ZERO, 
                Vec3::ZERO,
                35.0,
                55.0,
                [0,0]
            );

        let mut renderer = Self {
            width: 0,
            height: 0,
            frame_buffer: vec![],
            active_camera : Arc::new(RwLock::new(camera)),
            last_render_time: Duration::from_secs(0),
        };

        renderer.init_scene();

        renderer
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

    fn init_scene(&mut self) {
        let mut cam = self.active_camera.write().unwrap();
        (*cam).set_position( Vec3::new(0.0, 0.0, 2.0));
    }

    fn set_size(&mut self, size: [u32; 2]) {
        self.width = size[0];
        self.height = size[1];
        let mut cam = self.active_camera.write().unwrap();
        (*cam).set_image_resolutions(size);
    }

    pub fn render(&mut self, scene: &Scene, width: u32, height: u32) {
        let render_start_time = Instant::now();

        self.set_size([width, height]);

        self.frame_buffer = vec![0xFFFFFFFF_u32; (width * height) as usize];
        // let aspect_ratio = width as f32 / height as f32;

        for y in 0..height {
            for x in 0..width {
                let cam = self.active_camera.read().unwrap();

                let ray = (*cam).get_ray(x, y);

                let color = convert_to_argb(
                    &Self::trace_ray(ray, scene)
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

    fn trace_ray(ray : Ray, scene: &Scene) -> Vec4 /* returns color */ {
        // (bx^2 + by^2 + bz^2)t^2 + 2(axbx + ayby + azbz)t + (ax^2 + ay^2 + az^2 - r^2)
        // a vec ray origin
        // b vec ray direction
        // r radius
        // t hit distance

        if scene.spheres.is_empty() {
            return Vec4::new(0.0, 0.0, 0.0, 1.0);
        }

        let mut hit_distance = f32::MAX;
        let mut closest_sphere : Option<&Sphere> = None;

        for sphere in scene.spheres.iter() {

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

            if closest_t < 0.0 {
                continue;
            }

            if closest_t < hit_distance {
                closest_sphere = Some(sphere);
                hit_distance = closest_t;
            }
        }

        if let Some(sphere) = closest_sphere {
            let light_origin = Vec3::new(-2.0, 1.0, 2.0);
            let light_direction = (Vec3::ZERO - light_origin).normalize();

            let origin = ray.origin - sphere.position;
            let hit_point = origin + hit_distance * ray.direction;

            let normal = hit_point.normalize();

            let shading_factor = normal.dot(-light_direction).max(0.0);

            // return Vec4::from(((normal * 0.5 + 0.5 ) * shading_factor, 1.0));
            Vec4::from((sphere.albedo * shading_factor, 1.0))
        }       
        else
        {
            Vec4::new(0.0, 0.0, 0.0, 1.0)
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


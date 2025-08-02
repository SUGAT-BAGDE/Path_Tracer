use std::default::Default;

use std::time::{Duration, Instant};

use glam::{Vec3, Vec4};

use crate::ray_tracer::camera::Camera;
use crate::ray_tracer::Ray;
use crate::ray_tracer::utils::convert_to_argb;

pub struct RayTracer {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    pub camera: Camera,
    last_render_time: Duration,
}

impl RayTracer {
    pub fn new() -> Self {
        let mut renderer = Self {
            width: 0,
            height: 0,
            pixels: vec![],
            last_render_time: Duration::from_secs(0),
            camera : Camera::new(
                Vec3::ZERO, 
                Vec3::ZERO,
                35.0,
                55.0,
                [0,0]
            )
        };

        renderer.init_scene();

        return renderer;
    }

    pub fn get_current_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    pub fn prepare_pixels(&mut self, width: u32, height: u32) {
        // if self.pixels.len() == 0 {
        if self.pixels.len() != (width * height) as usize
            || self.width != width
            || self.height != height
        {
            self.render(width, height);
        }
    }

    fn init_scene(&mut self) {
        self.camera.position = Vec3::new(0.0, 0.0, 2.0);
    }

    fn set_size(&mut self, size: [u32; 2]) {
        self.width = size[0];
        self.height = size[1];
        self.camera.set_image_resolutions(size);
    }

    pub fn render(&mut self, width: u32, height: u32) {
        let render_start_time = Instant::now();

        self.set_size([width, height]);

        self.pixels = vec![0xFFFFFFFF_u32; (width * height) as usize];
        // let aspect_ratio = width as f32 / height as f32;

        for y in 0..height {
            for x in 0..width {

                let ray = self.camera.get_ray(x, y);
                // let ray = Ray { origin: ray_origin, direction: ray_direction };

                let color = convert_to_argb(
                    &Self::trace_ray(ray)
                        .clamp(
                            Vec4::from((0.0, 0.0, 0.0, 0.0)),
                            Vec4::from((1.0, 1.0, 1.0, 1.0))
                        ),
                );

                self.pixels[(y * width + x) as usize] = color;
            }
        }

        self.last_render_time = render_start_time.elapsed();
    }

    fn trace_ray(ray : Ray) -> Vec4 /* returns color */ {
        // (bx^2 + by^2 + bz^2)t^2 + 2(axbx + ayby + azbz)t + (ax^2 + ay^2 + az^2 - r^2)
        // a vec ray origin
        // b vec ray direction
        // r radius
        // t hit distance


        let radius = 1.0;
        let sphere_color = Vec3::new(1.0, 0.0, 1.0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(ray.origin);
        let c = ray.origin.dot(ray.origin) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec4::new(0.0, 0.0, 0.0, 1.0);
        }

        let light_origin = Vec3::new(-2.0, 1.0, 2.0);
        let light_direction = (Vec3::ZERO - light_origin).normalize();

        let sqrt_d = discriminant.sqrt();
        let t0 = (-b + sqrt_d) / (2.0 * a);
        let t1 = (-b - sqrt_d) / (2.0 * a);
        // let t = t0.min(t1);

        let t = match (t0 > 0.0, t1 > 0.0) {
            (true, true) => t0.min(t1),   // Both positive: take closer one
            (true, false) => t0,
            (false, true) => t1,
            (false, false) => return Vec4::new(0.0, 0.0, 0.0, 1.0), // Both behind the ray
        };

        let mut normal = ray.origin + t * ray.direction;

        normal = normal.normalize();

        let shading_factor = normal.dot(-light_direction).max(0.0);

        // return Vec4::from(((normal * 0.5 + 0.5 ) * shading_factor, 1.0));
        Vec4::from((sphere_color * shading_factor, 1.0))
    }

    pub fn get_output(&self) -> &[u32] {
        self.pixels.as_slice()
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

use std::default::Default;

use std::time::{Duration, Instant};

use glam::{Vec2, Vec3, Vec4};

use crate::ray_tracer::utils::convert_to_argb;

pub struct RayTracer {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    last_render_time: Duration,
}

impl RayTracer {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            pixels: vec![],
            last_render_time: Duration::from_secs(0),
        }
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

    fn set_size(&mut self, size: [u32; 2]) {
        self.width = size[0];
        self.height = size[1];
    }

    pub fn render(&mut self, width: u32, height: u32) {
        let render_start_time = Instant::now();

        self.set_size([width, height]);

        self.pixels = vec![0xFFFFFFFF_u32; (width * height) as usize];
        let aspect_ratio = width as f32 / height as f32;

        for y in 0..height {
            for x in 0..width {
                let mut vec = Vec2::new((x as f32) / (width as f32), (y as f32) / (height as f32));
                vec = vec * 2.0 - 1.0;
                vec.x *= aspect_ratio;

                let color = convert_to_argb(
                    &Self::per_pixel(vec)
                        // &self.per_pixel(vec)
                        .clamp(
                            Vec4::from((0.0, 0.0, 0.0, 0.0)),
                            Vec4::from((1.0, 1.0, 1.0, 1.0)),
                        ),
                );

                self.pixels[(y * width + x) as usize] = color;
            }
        }

        self.last_render_time = render_start_time.elapsed();
    }

    fn per_pixel(coord: Vec2) -> Vec4 /* returns color */ {
        // (bx^2 + by^2 + bz^2)t^2 + 2(axbx + ayby + azbz)t + (ax^2 + ay^2 + az^2 - r^2)
        // a vec ray origin
        // b vec ray direction
        // r radius
        // t hit distance

        // let ray_origin = Vec3::new(0.0, 2.0, 0.0);
        let ray_origin = Vec3::new(0.0, 0.0, 2.0);
        // let ray_direction = Vec3::new(coord.x, -1.0, coord.y);
        let ray_direction = Vec3::new(coord.x, coord.y, -1.0);
        let radius = 1.0;
        let sphere_color = Vec3::new(1.0, 0.0, 1.0);

        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * ray_direction.dot(ray_origin);
        let c = ray_origin.dot(ray_origin) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec4::new(0.0, 0.0, 0.0, 1.0);
        }

        let light_origin = Vec3::new(-2.0, 1.0, 2.0);
        let light_direction = (Vec3::ZERO - light_origin).normalize();

        let sqrt_d = discriminant.sqrt();
        let t0 = (-b + sqrt_d) / (2.0 * a);
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t = t0.min(t1);

        let mut normal = ray_origin + t * ray_direction;

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

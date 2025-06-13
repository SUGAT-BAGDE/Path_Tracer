use std::{default::Default};

use std::time::{Duration, Instant};

use glam::{Vec2, Vec3A};
pub struct RayTracer {
    width : u32,
    height : u32,
    pixels : Vec<u32>,
    last_render_time : Duration
}

impl RayTracer {
    pub fn new() -> Self {
        Self {
            width : 0,
            height : 0,
            pixels : vec![],
            last_render_time : Duration::from_secs(0)
        }
    }

    pub fn get_current_size(&self) -> [u32; 2] {
        return [self.width, self.height];
    }

    pub fn prepare_pixels(&mut self, width : u32, height : u32){        
        // if self.pixels.len() == 0 {
        if self.pixels.len() != (width*height) as usize || self.width!= width || self.height != height {
            self.render(width, height);
        }
    }

    fn set_size(&mut self, size: [u32; 2]){
        self.width = size[0];
        self.height = size[1];
    }

    pub fn render(&mut self, width : u32, height : u32) {
        let render_start_time = Instant::now();

        self.set_size([width, height]);
        
        self.pixels = vec![0xFFFFFFFF_u32; (width * height) as usize];
        let aspect_ratio = width as f32 / height as f32;

        for y in 0..height {
            for x in 0..width {
                let mut vec = Vec2::new((x as f32) / (width as f32), (y as f32)/ (height as f32));
                vec = vec * 2.0 - 1.0;
                vec.x = vec.x * aspect_ratio;
                self.pixels[(y*width + x) as usize] = Self::per_pixel(vec);
            }
        }

        self.last_render_time = render_start_time.elapsed();
    }

    fn per_pixel(coord : Vec2) -> u32 /* returns color */ {

        // (bx^2 + by^2 + bz^2)t^2 + 2(axbx + ayby + azbz)t + (ax^2 + ay^2 + az^2 - r^2)
        // a vec ray origin
        // b vec ray direction
        // r radius
        // t hit distance
        
        let ray_origin = Vec3A::new(0.0, 2.0, 0.0);
        // let ray_origin = Vec3::new(0.0, -2.0, 0.0);
        let ray_direction = Vec3A::new(coord.x, -1.0, coord.y);
        // let ray_direction = Vec3::new(coord.x, 1.0, coord.y);
        let radius = 1.0;
        
        let light_origin = Vec3A::new(-2.0, 1.0, 2.0);
        // let light_direction = Vec3A::new(-1.0, -1.0, -1.0).normalize();
        let light_direction = (Vec3A::ZERO - light_origin).normalize();

        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * ray_direction.dot(ray_origin);
        let c = ray_origin.dot(ray_origin) - radius*radius;

        let discriminant = b*b - 4.0*a*c;
        
        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            let t0 = (- b + sqrt_d)/(2.0*a);
            let t1 = (- b - sqrt_d)/(2.0*a);
            let t = t0.min(t1);
            
            let mut normal = ray_origin + t * ray_direction;

            normal = normal.normalize();
    
            let r = (037.0 * (normal.dot(-light_direction) as f32)) as u8;
            let g = (150.0 * (normal.dot(-light_direction) as f32)) as u8;
            let b = (190.0 * (normal.dot(-light_direction) as f32)) as u8;
            // let r = ((normal.x * 0.5 + 0.5) * 255.0 * (normal.dot(-light_direction))) as u8;
            // let g = ((normal.y * 0.5 + 0.5) * 255.0 * (normal.dot(-light_direction))) as u8;
            // let b = ((normal.z * 0.5 + 0.5) * 255.0 * (normal.dot(-light_direction))) as u8;
    
            return 0xFF000000_u32 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
        else {
            return 0xFF000000_u32;   
        }
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
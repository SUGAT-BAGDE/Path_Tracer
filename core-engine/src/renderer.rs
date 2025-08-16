use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use glam::{Vec3};

use crate::accumulator::Accumulator;
use crate::cameras::PinholeCamera;
use crate::cameras::Camera;
use crate::integrator::Integrator;
use crate::sampler::Sampler;
use crate::scene::{Scene};
use crate::utils::convert_to_argb;

pub struct RayTracer {
    width: u32,
    height: u32,
    frame_buffer : Vec<u32>,
    last_render_time: Duration,

    pub active_camera: Arc<RwLock<dyn Camera>>,
    // pub scene : Arc<Scene>

    integrator : Integrator,
    accumulator : Accumulator
}


impl RayTracer {
    pub fn new(width: u32, height: u32) -> Self {
        let camera =  PinholeCamera::new(
                Vec3::new(0.0, 0.0, 2.0), 
                Vec3::ZERO,
                35.0,
                55.0,
                [width,height]
            );

        let integrator = Integrator {
            bounces : 5,
            max_compulsory_bounces : 2
        };

        Self {
            width: 0,
            height: 0,
            frame_buffer: vec![],
            active_camera : Arc::new(RwLock::new(camera)),
            last_render_time: Duration::from_secs(0),

            accumulator : Accumulator::new(width, height),
            integrator
        }
    }

    pub fn set_active_camera(&mut self, camera : Arc<RwLock<dyn Camera>>) {
        self.active_camera = camera;
    }

    pub fn get_current_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    #[inline]
    pub fn prepare_pixels(&mut self, scene: &Scene, width: u32, height: u32) {
        self.render(scene, width, height, true);
    }

    fn set_size(&mut self, size: [u32; 2]) {
        self.width = size[0];
        self.height = size[1];

        if self.accumulator.get_resolution() != size {
            self.accumulator = Accumulator::new(size[0], size[1]);
        }

        let mut cam = self.active_camera.write().unwrap();
        cam.set_image_resolutions(size);
        drop(cam);
    }

    pub fn render(&mut self, scene: &Scene, width: u32, height: u32, acc : bool) {
        let render_start_time = Instant::now();

        self.set_size([width, height]);
        if !acc {
            self.accumulator = Accumulator::new(width, height);
        }

        self.frame_buffer = vec![0xFF000000_u32; (width * height) as usize];
        
        // init thread local accumulator
        let mut accumulator = Accumulator::new(width, height);
        let mut sampler = Sampler::new();
        for y in 0..height {
            for x in 0..width {

                let color = self.integrator
                    .compute_incomming_radience(scene, x, y, &self.active_camera, &mut sampler);

                accumulator.accumulate(x, y, color);
                self.frame_buffer[(y * width + x) as usize] = self.accumulator.get_color_argb(x, y);
            }
        }

        self.accumulator.merge(accumulator);

        self.last_render_time = render_start_time.elapsed();
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
        Self::new(0, 0)
    }
}


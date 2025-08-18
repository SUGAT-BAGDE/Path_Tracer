use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crossbeam::channel::Receiver;
use glam::Vec3;

use crate::accumulator::Accumulator;
use crate::cameras::{PinholeCamera, SharedCamera};
use crate::concurrency::Threadpool;
use crate::integrator::Integrator;
use crate::sampler::Sampler;
use crate::scene::Scene;

pub struct RayTracer {
    width: u32,
    height: u32,
    frame_buffer: Vec<u32>,
    last_render_time: Duration,

    pub active_camera: SharedCamera,
    // pub scene : Arc<Scene>
    integrator: Integrator,
    accumulator: Arc<RwLock<Accumulator>>,
    threadpool: Option<Threadpool>,
    threadpool_result_rx: Option<Receiver<Accumulator>>,
    merger_thread: Option<JoinHandle<()>>,
}

impl RayTracer {
    pub fn new(width: u32, height: u32) -> Self {
        let camera = PinholeCamera::new(
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::ZERO,
            35.0,
            55.0,
            [width, height],
        );

        let integrator = Integrator {
            bounces: 5,
            max_compulsory_bounces: 2,
        };
        let accumulator = Accumulator::new(width, height);
        let shared_acc = Arc::new(RwLock::new(accumulator));

        let (tp, result_rx) = Threadpool::new(3);
        let accum = Arc::clone(&shared_acc);
        let rx = result_rx.clone();
        let merger = thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(result_acc) => {
                        let mut acc = accum.write().unwrap();
                        acc.merge(result_acc);
                        drop(acc);
                    }
                    Err(e) => {
                        println!("Error receiving accumulator: {:?}", e);
                        break;
                    }
                }
            }
        });

        Self {
            width: 0,
            height: 0,
            frame_buffer: vec![],
            active_camera: Arc::new(RwLock::new(camera)),
            last_render_time: Duration::from_secs(0),
            accumulator: shared_acc,

            integrator,
            threadpool: Some(tp),
            threadpool_result_rx: Some(result_rx),
            merger_thread: Some(merger),
        }
    }

    pub fn set_active_camera(&mut self, camera: SharedCamera) {
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

        let mut accum_guard = self.accumulator.write().unwrap();
        if accum_guard.get_resolution() != size {
            *accum_guard = Accumulator::new(size[0], size[1]);
        }
        drop(accum_guard);

        let mut cam = self.active_camera.write().unwrap();
        cam.set_image_resolutions(size);
        drop(cam);
    }

    pub fn render(&mut self, scene: &Scene, width: u32, height: u32, acc: bool) {
        let render_start_time = Instant::now();

        self.set_size([width, height]);
        if !acc {
            let mut accum_guard = self.accumulator.write().unwrap();
            *accum_guard = Accumulator::new(width, height);
            drop(accum_guard);
        }

        // init thread local accumulator
        let mut accumulator = Accumulator::new(width, height);
        let mut sampler = Sampler::new();
        for y in 0..height {
            for x in 0..width {
                let color = self.integrator.compute_incomming_radience(
                    scene,
                    x,
                    y,
                    &self.active_camera,
                    &mut sampler,
                );

                accumulator.accumulate(x, y, color);
            }
        }

        let mut acc_guard = self.accumulator.write().unwrap();
        acc_guard.merge(accumulator);
        drop(acc_guard);

        self.last_render_time = render_start_time.elapsed();
    }

    pub fn get_output(&mut self) -> &[u32] {
        let accum_guard = self.accumulator.read().unwrap();
        accum_guard.write_to_image_buffer(&mut self.frame_buffer);
        drop(accum_guard);
        &self.frame_buffer
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

impl Drop for RayTracer {
    fn drop(&mut self) {
        if let Some(tp) = self.threadpool.take() {
            drop(tp);
        }
        if let Some(rx) = self.threadpool_result_rx.take() {
            drop(rx);
        }
        if let Some(thread) = self.merger_thread.take() {
            let _ = thread.join();
        }
    }
}

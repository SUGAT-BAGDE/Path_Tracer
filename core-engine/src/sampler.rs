use glam::{Vec2, Vec3};
use rand::{prelude::ThreadRng, Rng};

pub struct Sampler {
    rng : ThreadRng
}

impl Sampler {
    pub fn new() -> Self {
        Self {
            rng: rand::rng()
        }
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        self.rng.random::<f32>()
    }

    pub fn _next_2d(&mut self) -> Vec2 {
        Vec2::new(self.next_f32(), self.next_f32())
    }

    pub fn next_3d(&mut self) -> Vec3 {
        Vec3::new(self.next_f32(), self.next_f32(), self.next_f32()).normalize()
    }
}

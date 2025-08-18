use glam::{Vec2, Vec3};
use rand::{Rng, prelude::ThreadRng};

pub struct Sampler {
    rng: ThreadRng,
}

fn transform_local_to_world(local_dir: Vec3, normal: Vec3) -> Vec3 {
    let up = if normal.z.abs() < 0.999 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let tangent = up.cross(normal).normalize();
    let bitangent = normal.cross(tangent);

    local_dir.x * tangent + local_dir.y * bitangent + local_dir.z * normal
}

impl Sampler {
    pub fn new() -> Self {
        Self { rng: rand::rng() }
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        self.rng.random::<f32>()
    }

    pub fn _next_2d(&mut self) -> Vec2 {
        Vec2::new(self.next_f32(), self.next_f32())
    }

    pub fn _vec_3(&mut self, min: f32, max: f32) -> Vec3 {
        let range = max - min;
        Vec3::new(
            self.next_f32() * range + min,
            self.next_f32() * range + min,
            self.next_f32() * range + min,
        )
    }

    pub fn sample_hemisphere_cosine_weighted(&mut self, normal: Vec3) -> Vec3 {
        let r1 = self.next_f32();
        let r2 = self.next_f32();

        let phi = 2.0 * std::f32::consts::PI * r1;
        let r = r2.sqrt();

        let local_dir = Vec3::new(r * phi.cos(), r * phi.sin(), 1.0 - r);

        transform_local_to_world(local_dir, normal)
    }
}

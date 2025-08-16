use glam::{Vec4};

use crate::utils::convert_to_argb;

#[derive(Debug, Clone)]
pub struct Accumulator {
    width : u32,
    height : u32,
    pub framebuffer : Vec<Vec4>,
    pub sample_counts : Vec<u32>
}

impl Accumulator {
    pub fn new(width : u32, height : u32) -> Self {
        let size = (width * height) as usize;

        Self {
            width,
            height,
            framebuffer: vec![Vec4::ZERO; size],
            sample_counts: vec![0; size],
        }
    }

    pub fn get_resolution(&self) -> [u32; 2]{
        [self.width, self.height]
    }

    pub fn accumulate(&mut self, x : u32, y : u32, color : Vec4) {
        debug_assert!(x < self.width && y < self.height, "Pixel out of bounds");

        let index = (y * self.width + x) as usize;

        self.framebuffer[index] += color;
        self.sample_counts[index] += 1;
    }

    pub fn get_pixel_radiaence(&self, x: u32, y: u32) -> Vec4 {
        debug_assert!(x < self.width && y < self.height, "Pixel out of bounds");

        let index = (y * self.width + x) as usize;
        let color = self.framebuffer[index];
        let samples = self.sample_counts[index].max(1);

        color / samples as f32
    }

    pub fn get_color_argb(&self, x: u32, y: u32) -> u32 {
        debug_assert!(x < self.width && y < self.height, "Pixel out of bounds");

        let index = (y * self.width + x) as usize;
        let color = self.framebuffer[index];
        let samples = self.sample_counts[index].max(1);

        let mut averaged = color / samples as f32;

        // Tone mapping (Reinhard)
        averaged = averaged / (averaged + Vec4::ONE);

        // Gamma correction
        averaged = Vec4::new(
            averaged.x.powf(1.0 / 2.2),
            averaged.y.powf(1.0 / 2.2),
            averaged.z.powf(1.0 / 2.2),
            averaged.w, // usually alpha is 1.0, or pass-through
        );

        // Clamp to [0, 1]
        averaged = averaged.clamp(Vec4::ZERO, Vec4::ONE);

        convert_to_argb(&averaged)
    }

    pub fn to_image_buffer(&self) -> Vec<u32> {
        self.framebuffer
            .iter()
            .zip(&self.sample_counts)
            .map(|(color, &samples)| {
                if samples > 0 {
                    let avg_color = *color / samples as f32;
                    convert_to_argb(&avg_color)
                } else {
                    convert_to_argb(&Vec4::ZERO) // black if no samples
                }
            })
            .collect()
    }

    /// Merges two accumulators by summing corresponding pixels and sample counts.
    pub fn merge(&mut self, b: Self) {
        assert_eq!(self.width, b.width, "Widths do not match: \nself.width = {:?}\nb.width = {:?}", self.width, b.width);
        assert_eq!(self.height, b.height, "Heights do not match: \nself.height = {:?}\nb.height = {:?}", self.height, b.height);

        for (c1, c2) in self.framebuffer.iter_mut().zip(b.framebuffer) {
            *c1 += c2;
        }

        for (s1, s2) in self.sample_counts.iter_mut().zip(b.sample_counts) {
            *s1 += s2;
        }
    }
}

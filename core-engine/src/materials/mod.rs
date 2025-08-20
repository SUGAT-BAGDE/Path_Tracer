use glam::Vec3;

pub struct Matrial {
    pub albedo: Vec3,
    pub roughness: f32,
    pub metalic: f32,

    pub emission_color: Vec3,
    pub emissive_power: f32,
}

impl Default for Matrial {
    fn default() -> Self {
        Self {
            albedo: Vec3::ONE,
            roughness: 0.5,
            metalic: 0.0,

            emission_color: Vec3::ZERO,
            emissive_power: 0.0,
        }
    }
}

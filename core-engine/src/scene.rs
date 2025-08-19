use glam::Vec3;

use crate::file_formats::ExrImage;

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

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material_id: i32,
}


#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub materials: Vec<Matrial>,
    pub default_sky_color: Vec3,

    pub skybox: Option<ExrImage>,
}

impl Scene {
    pub fn get_example_scene() -> Self {
        let exr_img = ExrImage::load_exr_image("./assets/env/default_skybox_1.exr");
        if let Err(e) = &exr_img {
            println!(
                "Current working directory: {}",
                std::env::current_dir().unwrap().display()
            );
            eprintln!("Failed loading EXR: {}", e);
        }

        let mut scene = Self {
            spheres: vec![],
            materials: vec![],
            default_sky_color: Vec3::new(0.6, 0.7, 0.9),

            skybox: exr_img.ok(),
        };

        {
            let material = Matrial {
                albedo: Vec3::new(1.0, 0.0, 1.0),
                ..Default::default()
            };
            scene.materials.push(material);

            let sphere = Sphere {
                position: Vec3::new(0.0, 0.0, 0.0),
                radius: 0.5,
                material_id: 0,
            };
            scene.spheres.push(sphere);
        }

        {
            let material = Matrial {
                albedo: Vec3::new(0.2, 0.3, 1.0),
                ..Default::default()
            };
            scene.materials.push(material);

            let sphere = Sphere {
                position: Vec3::new(0.0, -100.5, 0.0),
                radius: 100.0,
                material_id: 1,
            };
            scene.spheres.push(sphere);
        }

        scene
    }
}

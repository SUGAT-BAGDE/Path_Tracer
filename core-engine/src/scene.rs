use glam::Vec3;

use crate::file_formats::ExrImage;

use crate::geometry::triangle::Triangle;
use crate::geometry::Sphere;
use crate::materials::Matrial;

#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub triangles: Vec<Triangle>,

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
            ..Default::default()
        };

        {
            let material = Matrial {
                albedo: Vec3::new(1.0, 0.0, 1.0),
                ..Default::default()
            };
            scene.materials.push(material);

            let sphere = Sphere {
                position: Vec3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                material_id: 0,
            };
            scene.spheres.push(sphere);
        }

        // {
        //     let material = Matrial {
        //         albedo: Vec3::new(0.2, 0.3, 1.0),
        //         ..Default::default()
        //     };
        //     scene.materials.push(material);

        //     let sphere = Sphere {
        //         position: Vec3::new(0.0, -100.5, 0.0),
        //         radius: 100.0,
        //         material_id: 1,
        //     };
        //     scene.spheres.push(sphere);
        // }

        scene
    }
}

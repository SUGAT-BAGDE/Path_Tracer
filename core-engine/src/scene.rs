use exr::prelude::*;
use glam::Vec3;

pub struct Matrial {
    pub albedo : Vec3,
    pub roughness : f32,
    pub metalic : f32,

    pub emission_color : Vec3,
    pub emissive_power : f32
}

impl Default for Matrial {
    fn default() -> Self {
        Self {
            albedo: Vec3::ONE,
            roughness: 0.5,
            metalic: 0.0,
            
            emission_color : Vec3::ZERO,
            emissive_power : 0.0
        }
    }
}

pub struct Sphere {
    pub position : Vec3,
    pub radius : f32,
    pub material_id : i32
}

/// HDR skybox loaded from EXR
#[derive(Default)]
pub struct ExrImage {
    pub pixels_buffer: Vec<Vec3>,
    pub width: usize,
    pub height: usize,
}

impl ExrImage {
    pub fn sample(&self, dir: Vec3) -> Vec3 {
        let dir = dir.normalize();
        let theta = dir.y.clamp(-1.0, 1.0).acos();
        let phi = dir.z.atan2(dir.x);
        let u = (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        let v = theta / std::f32::consts::PI;

        let x = (u * self.width as f32).floor() as usize % self.width;
        let y = (v * self.height as f32).floor() as usize % self.height;

        self.pixels_buffer[y * self.width + x]
    }
}

pub fn load_exr_image(path: &str) -> Result<ExrImage> {
    let image_2d = read_first_rgba_layer_from_file(
        path,
        |resolution, _| {
            let image_data = vec![Vec3::ZERO; resolution.width() * resolution.height()];
            ExrImage {
                pixels_buffer : image_data,
                width : resolution.width(),
                height : resolution.height()
            }
        },
        |skybox, pos, (r, g, b, _a) : (f32, f32, f32, f32)| {
             skybox.pixels_buffer[pos.y() * skybox.width + pos.x()] = Vec3::new(r, g, b);
        },
    );

    match image_2d {
        Ok(img) => {
            let skybox = img.layer_data.channel_data.pixels;    
            Ok(skybox)
        },
        Err(err) => {
            Err(err)
        }
    }
}

#[derive(Default)]
pub struct Scene {
    pub spheres : Vec<Sphere>,
    pub materials : Vec<Matrial>,
    pub default_sky_color : Vec3,

    pub skybox : Option<ExrImage>
}

impl Scene {
    pub fn get_example_scene() -> Self {
        let exr_img = load_exr_image("./assets/env/default_skybox_1.exr");
        if let Err(e) = &exr_img {
            println!("Current working directory: {}", std::env::current_dir().unwrap().display());
            eprintln!("Failed loading EXR: {}", e);
        }
        
        let mut scene = Self {
            spheres : vec![],
            materials : vec![],
            default_sky_color : Vec3::new(0.6, 0.7, 0.9),

            skybox : exr_img.ok()
        };
        
        {
            let mut material = Matrial::default();
            material.albedo = Vec3::new(1.0, 0.0, 1.0);
            scene.materials.push(material);

            let sphere = Sphere{
                position : Vec3::new(0.0, 0.0, 0.0),
                radius : 0.5,
                material_id : 0
            };
            scene.spheres.push(sphere);
        }

        {
            let mut material = Matrial::default();
            material.albedo = Vec3::new(0.2, 0.3, 1.0);
            scene.materials.push(material);

            let sphere = Sphere{
                position : Vec3::new(0.0, -100.5, 0.0),
                radius : 100.0,
                material_id : 1
            };
            scene.spheres.push(sphere);
        }

        scene
    }
}

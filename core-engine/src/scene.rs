use glam::Vec3;

pub struct Matrial {
    pub albedo : Vec3,
    pub roughness : f32,
    pub metalic : f32,
}

impl Default for Matrial {
    fn default() -> Self {
        Self {
            albedo: Vec3::ONE,
            roughness: 0.5,
            metalic: 0.0
        }
    }
}

pub struct Sphere {
    pub position : Vec3,
    pub radius : f32,
    pub material_id : i32
}

#[derive(Default)]
pub struct Scene {
    pub spheres : Vec<Sphere>,
    pub materials : Vec<Matrial>,
    pub sky_color : Vec3
}

impl Scene {
    pub fn get_example_scene() -> Self {
        let mut scene = Self {
            spheres : vec![],
            materials : vec![],
            sky_color : Vec3::new(0.6, 0.7, 0.9)
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

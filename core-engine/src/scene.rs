use glam::Vec3;

pub struct Sphere {
    pub position : Vec3,
    pub radius : f32,
    pub albedo : Vec3
}

#[derive(Default)]
pub struct Scene {
    pub spheres : Vec<Sphere>
}

impl Scene {
    pub fn get_example_scene() -> Self {
        let mut scene = Self {
            spheres : vec![]
        };
        
        {
            let sphere = Sphere{
                position : Vec3::new(0.0, 0.0, 0.0),
                radius : 0.5,
                albedo : Vec3::new(1.0, 0.0, 1.0)
            };
            scene.spheres.push(sphere);
        }

        {
            let sphere = Sphere{
                position : Vec3::new(-1.0, 0.0, -5.0),
                radius : 2.0,
                albedo : Vec3::new(0.0, 0.0, 1.0)
            };
            scene.spheres.push(sphere);
        }

        scene
    }
}

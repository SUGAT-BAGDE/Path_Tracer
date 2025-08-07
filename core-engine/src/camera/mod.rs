use glam::Vec3;

use crate::ray::Ray;

pub trait Camera {
    fn get_ray(&self, x : u32, y : u32) -> Ray;
    fn set_position(&mut self, position : Vec3);
    fn set_rotation(&mut self, rotation : Vec3);
    fn set_image_resolutions(&mut self, image_resolution : [u32; 2]);

    fn compute_transformation_matrix(&mut self);
    fn on_update(&mut self);
}

pub mod pinhole_camera;
pub use pinhole_camera::PinholeCamera;

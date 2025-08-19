use std::sync::Arc;
use std::sync::RwLock;

use imgui::Ui;

use insploray::renderer::RayTracer;
use insploray::scene::Scene;
use insploray::cameras::Camera;
use insploray::cameras::PinholeCamera;
use insploray::scene::{Sphere, Matrial};
use insploray::Vec3;

pub struct Viewport {
    pub renderer : RayTracer,
    pub scene : Arc<RwLock<Scene>>,
    pub camera : Arc<RwLock<PinholeCamera>>
}

impl Viewport {
    pub fn draw_scene_setting_window(&mut self, ui : &Ui, viewport_size: &[f32; 2]) {
        // /*
        let mut update = false;
        if let Ok(mut scene) = self.scene.try_write() {

        ui.window("Scene Settings")
            .size([300.0, 400.0], imgui::Condition::FirstUseEver)
            .position([200.0, 500.0], imgui::Condition::FirstUseEver)
            .build(|| {
                for i in 0..scene.spheres.len() {
                    let _id = ui.push_id_usize(i);

                    update |= ui.input_float3("Position", &mut scene.spheres[i].position)
                        .build();
                    update |= imgui::Drag::new("Radius").range(0.0, f32::MAX)
                        .speed(0.05)
                        .build(ui, &mut scene.spheres[i].radius);
                    update |= imgui::Drag::new("Material")
                        .range(-1, scene.materials.len() as i32 - 1)
                        .build(ui, &mut scene.spheres[i].material_id);

                    ui.separator();
                }

                if ui.button("Add sphere") {
                    let sphere = Sphere{
                        position : Vec3::ZERO,
                        radius : 1.0,
                        material_id : -1, 
                    };
                    scene.spheres.push(sphere);
                    update |= true;
                }
                ui.separator();
                ui.separator();

                for i in 0..scene.materials.len() {
                    let _id = ui.push_id_usize(i);

                    update |= ui.color_edit3("Albedo", &mut scene.materials[i].albedo);
                    update |= imgui::Drag::new("Roughness").range(0.0, 1.0)
                        .speed(0.005)
                        .build(ui, &mut scene.materials[i].roughness);
                    update |= imgui::Drag::new("Metalic").range(0.0, 1.0)
                        .speed(0.005)
                        .build(ui, &mut scene.materials[i].metalic);
                    update |= ui.color_edit3("Emission Color", &mut scene.materials[i].emission_color);
                    update |= imgui::Drag::new("Emissive Power").range(0.0, 1.0)
                        .build(ui, &mut scene.materials[i].emissive_power);

                    ui.separator();
                }

                if ui.button("Add Materal") {
                    let material = Matrial::default();
                    scene.materials.push(material);
                    update |= true;
                }

                update |= ui.color_edit3("Sky color", &mut scene.default_sky_color);
            });
        drop(scene);
        

        if update {
            self.renderer.render_updated(&self.scene,
                viewport_size[0] as u32,
                viewport_size[1] as u32,
            );
        }
    }
        else {
            println!("Skipping Scene Setting window in this frame!")
        }
    }

    pub fn handle_input(
        &mut self,
        ui : &Ui,
        width : u32,
        height : u32,
    ) {
        let delta_time = ui.io().delta_time;
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        let move_speed = 0.25 * delta_time * 30.0;
        let mouse_sensitivity = 0.0005 * delta_time * 30.0; 
        // radians per pixel
        let camera = self.camera.read().unwrap();

        let forward = camera.forward;
        let up = camera.up;
        let right = camera.right;

        let key_moves = [
            (imgui::Key::W,  forward),
            (imgui::Key::S, -forward),
            (imgui::Key::D,  right),
            (imgui::Key::A, -right),
            (imgui::Key::E,  up),
            (imgui::Key::Q, -up),
        ];

        let mut new_rotation = camera.rotation;
        let mut new_position = camera.position;

        drop(camera);

        let mut moved = false;
        for (key, dir) in key_moves {
            if ui.is_key_down(key) {
                
                new_position += move_speed * dir;
                self.camera.write().unwrap()
                    .set_position(new_position);
                moved = true;
            }
        }

        if ui.is_mouse_down(imgui::MouseButton::Right) {
            ui.set_mouse_cursor(None);

            let delta = ui.mouse_drag_delta_with_button(imgui::MouseButton::Right);
            if delta != [0.0, 0.0] {
                new_rotation.y -= delta[0] * mouse_sensitivity;
                new_rotation.x -= delta[1] * mouse_sensitivity;

                new_rotation.x = new_rotation.x.clamp(-max_pitch, max_pitch);

                self.camera.write().unwrap()
                    .set_rotation(new_rotation);
                moved = true;
            }
        }
        else {
            ui.set_mouse_cursor(Some(imgui::MouseCursor::Arrow));
        }

        if moved {
            self.renderer
                .render_updated(&self.scene, width, height);
        }

    }


}

impl Default for Viewport {
    fn default() -> Self {
        let position =Vec3::new(0.0, 0.0, 2.0);
        let camera =  Arc::new(RwLock::new(
            PinholeCamera::new(
                position, 
                Vec3::ZERO,
                35.0,
                55.0,
                [0,0]
            )
        ));

        let mut renderer = RayTracer::new(0, 0);
        renderer.set_active_camera(camera.clone());
        let scene = Arc::new(RwLock::new(Scene::get_example_scene()));

        Self {
            camera,
            renderer,
            scene
        }
    }
}

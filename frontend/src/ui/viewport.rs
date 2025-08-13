use std::sync::Arc;
use std::sync::RwLock;

use imgui::Ui;

use insploray::renderer::RayTracer;
use insploray::scene::Matrial;
use insploray::scene::Scene;
use insploray::camera::Camera;
use insploray::camera::PinholeCamera;
use insploray::scene::Sphere;
use insploray::Vec3;

pub struct Viewport {
    pub renderer : RayTracer,
    pub scene : Scene,
    pub camera : Arc<RwLock<PinholeCamera>>
}

impl Viewport {
    pub fn draw_scene_setting_window(&mut self, ui : &Ui, viewport_size: &[f32; 2]) {
        let mut update = false;

        ui.window("Scene Settings")
            .size([300.0, 400.0], imgui::Condition::FirstUseEver)
            .position([200.0, 500.0], imgui::Condition::FirstUseEver)
            .build(|| {
                for i in 0..self.scene.spheres.len() {
                    let _id = ui.push_id_usize(i);

                    update |= ui.input_float3("Position", &mut self.scene.spheres[i].position)
                        .build();
                    update |= imgui::Drag::new("Radius").range(0.0, f32::MAX)
                        .speed(0.05)
                        .build(ui, &mut self.scene.spheres[0].radius);
                    update |= imgui::Drag::new("Material")
                        .range(-1, self.scene.materials.len() as i32 - 1)
                        .build(ui, &mut self.scene.spheres[0].material_id);

                    ui.separator();
                }

                if ui.button("Add sphere") {
                    let sphere = Sphere{
                        position : Vec3::ZERO,
                        radius : 1.0,
                        material_id : -1, 
                    };
                    self.scene.spheres.push(sphere);
                    update |= true;
                }
                ui.separator();
                ui.separator();

                for i in 0..self.scene.materials.len() {
                    let _id = ui.push_id_usize(i);

                    update |= ui.color_edit3("Albedo", &mut self.scene.materials[i].albedo);
                    update |= imgui::Drag::new("Roughness").range(0.0, 1.0)
                        .speed(0.005)
                        .build(ui, &mut self.scene.materials[i].roughness);
                    update |= imgui::Drag::new("Metalic").range(0.0, 1.0)
                        .speed(0.005)
                        .build(ui, &mut self.scene.materials[i].metalic);

                    ui.separator();
                }

                if ui.button("Add Materal") {
                    let material = Matrial::default();
                    self.scene.materials.push(material);
                    update |= true;
                }

                update |= ui.color_edit3("Sky color", &mut self.scene.sky_color);
            });

        if update {
            self.renderer.render(&self.scene,
                viewport_size[0] as u32,
                viewport_size[1] as u32);
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
                .render(
                    &self.scene,
                    width, 
                    height
                );
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

        let mut renderer = RayTracer::new();
        renderer.set_active_camera(camera.clone());

        Self {
            camera : camera,
            renderer : renderer,
            scene: Scene::get_example_scene()
        }
    }
}

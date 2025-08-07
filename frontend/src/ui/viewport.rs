use imgui::Ui;

use insploray::renderer::RayTracer;
use insploray::scene::Scene;
use insploray::camera::Camera;
use insploray::camera::PinholeCamera;

pub struct Viewport {
    pub renderer : RayTracer,
    pub scene : Scene,
    pub camera : PinholeCamera
}

impl Viewport {

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

        let forward = self.renderer.active_camera.forward;
        let up = self.renderer.active_camera.up;
        let right = self.renderer.active_camera.right;

        let key_moves = [
            (imgui::Key::W,  forward),
            (imgui::Key::S, -forward),
            (imgui::Key::D,  right),
            (imgui::Key::A, -right),
            (imgui::Key::E,  up),
            (imgui::Key::Q, -up),
        ];

        let mut moved = false;
        for (key, dir) in key_moves {
            if ui.is_key_down(key) {
                self.renderer.active_camera
                    .set_position(self.renderer.active_camera.position + move_speed * dir);
                moved = true;
            }
        }

        if ui.is_mouse_down(imgui::MouseButton::Right) {
            ui.set_mouse_cursor(None);

            let delta = ui.mouse_drag_delta_with_button(imgui::MouseButton::Right);
            if delta != [0.0, 0.0] {
                let mut new_rotation = self.renderer.active_camera.rotation;
                new_rotation.y -= delta[0] * mouse_sensitivity;
                new_rotation.x -= delta[1] * mouse_sensitivity;

                new_rotation.x = new_rotation.x.clamp(-max_pitch, max_pitch);

                self.renderer.active_camera.set_rotation(new_rotation);
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
        Self {
            camera : PinholeCamera::default(),
            renderer : RayTracer::default(),
            scene: Scene::get_example_scene()
        }
    }
}

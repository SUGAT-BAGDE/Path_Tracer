use std::time::Instant;

use glam::Vec2;

use imgui::TextureId;
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use wgpu::wgt::TextureViewDescriptor;

use crate::ray_tracer::renderer::RayTracer;

use crate::ui::app_window::AppWindow;
use crate::ui::utils::create_texture_from_pixels;

#[derive(Default)]
pub struct App {
    window: Option<AppWindow>,
    viewport_renderer: RayTracer,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(AppWindow::new(event_loop))
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();

        match event {
            WindowEvent::Resized(size) => {
                window.surface_desc.width = size.width;
                window.surface_desc.height = size.height;

                window
                    .surface
                    .configure(&window.device, &window.surface_desc);
            }
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                // let delta_s = imgui.last_frame.elapsed();
                let now = Instant::now();

                imgui
                    .context
                    .io_mut()
                    .update_delta_time(now - imgui.last_frame);

                imgui.last_frame = now;

                let frame = match window.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("dropped frame: {e:?}");
                        return;
                    }
                };

                imgui
                    .platform
                    .prepare_frame(imgui.context.io_mut(), &window.window)
                    .expect("Failed to prepare frame");

                let ui = imgui.context.frame();
                let mut frame_id: Option<TextureId> = None;

                // Lets create a window
                {
                    // write docking mechanism
                    ui.dockspace_over_main_viewport();

                    let mut viewport_size: [f32; 2] = [0.0, 0.0];

                    let style_guard = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
                    let imgui_window = ui.window("Viewport");
                    imgui_window
                        .size([500.0, 500.0], imgui::Condition::FirstUseEver)
                        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
                        .build(|| {
                            viewport_size = ui.content_region_avail();
                            let width = viewport_size[0] as u32;
                            let height = viewport_size[1] as u32;

                            if ui.is_window_focused() {

                                let camera = &mut self.viewport_renderer.camera;
                                let move_speed = 0.25;
                                let mouse_sensitivity = 0.0005; // radians per pixel
                                let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;

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

                                let mut moved = false;
                                for (key, dir) in key_moves {
                                    if ui.is_key_down(key) {
                                        camera.set_position(camera.position + move_speed * dir);
                                        moved = true;
                                    }
                                }

                                if ui.is_mouse_down(imgui::MouseButton::Right) {
                                    ui.set_mouse_cursor(None);

                                    let delta = ui.mouse_drag_delta_with_button(imgui::MouseButton::Right);
                                    if delta != [0.0, 0.0] {
                                        let mut new_rotation = camera.rotation.clone();
                                        new_rotation.y -= delta[0] * mouse_sensitivity;
                                        new_rotation.x -= delta[1] * mouse_sensitivity;

                                        new_rotation.x = new_rotation.x.clamp(-max_pitch, max_pitch);

                                        camera.set_rotation(new_rotation);
                                        moved = true;
                                    }
                                }
                                else {
                                    ui.set_mouse_cursor(Some(imgui::MouseCursor::Arrow));
                                }



                                if moved {
                                    self.viewport_renderer
                                        .render(viewport_size[0] as u32, viewport_size[1] as u32);
                                }

                            }

                            self.viewport_renderer.prepare_pixels(width, height);

                            let [c_w, c_h] = self.viewport_renderer.get_current_size();

                            let texture_id = create_texture_from_pixels(
                                self.viewport_renderer.get_output(),
                                self.viewport_renderer.get_current_size(),
                                &window.device,
                                &window.queue,
                                &mut imgui.renderer,
                            );

                            imgui::Image::new(texture_id.clone(), [c_w as f32, c_h as f32])
                                .uv0(Vec2::new(0.0, 1.0))
                                .uv1(Vec2::new(1.0, 0.0))
                                .build(&ui);

                            frame_id = Some(texture_id);
                        });

                    drop(style_guard);

                    let imgui_window = ui.window("Settings");
                    imgui_window
                        .size([300.0, 200.0], imgui::Condition::FirstUseEver)
                        .position([500.0, 200.0], imgui::Condition::FirstUseEver)
                        .build(|| {
                            let duration = self.viewport_renderer.get_last_render_time();
                            ui.text(format!("Last Render : {duration:?}"));
                            ui.text(format!("Image : {viewport_size:?}"));
                            ui.button("Render").then(|| {
                                self.viewport_renderer
                                    .render(viewport_size[0] as u32, viewport_size[1] as u32);
                            });

                            let focal_length = self.viewport_renderer.camera.focal_length;
                            if imgui::Drag::new("Focal length")
                                .build(&ui, focal_length) {
                                self.viewport_renderer.camera.set_focal_length(focal_length);
                                self.viewport_renderer
                                    .render(viewport_size[0] as u32, viewport_size[1] as u32);
                            }
                        });

                    // if ui.is_key_down(imgui::Key::W) {
                    //     println!("Pressed W");
                    //     println!("from {} to {}",
                    //         self.viewport_renderer.camera.position,
                    //         self.viewport_renderer.camera.position + 0.5 * self.viewport_renderer.camera.forward
                    //     );
                    //     self.viewport_renderer.camera.set_position(
                    //         self.viewport_renderer.camera.position + 0.5 * self.viewport_renderer.camera.forward
                    //     );
                    //     self.viewport_renderer
                    //         .render(viewport_size[0] as u32, viewport_size[1] as u32);
                    // }
                }

                let mut encoder = window
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                if imgui.last_cursor != ui.mouse_cursor() {
                    imgui.last_cursor = ui.mouse_cursor();
                    imgui.platform.prepare_render(&ui, &window.window);
                }

                let view = frame.texture.create_view(&TextureViewDescriptor::default());

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(imgui.clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                imgui
                    .renderer
                    .render(
                        imgui.context.render(),
                        &window.queue,
                        &window.device,
                        &mut rpass,
                    )
                    .expect("Rendering Failed!...");

                drop(rpass);

                imgui.renderer.textures.remove(frame_id.unwrap());

                window.queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => (),
        }

        imgui.platform.handle_event::<()>(
            imgui.context.io_mut(),
            &window.window,
            &Event::WindowEvent { window_id, event },
        );
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: ()) {
        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();
        imgui.platform.handle_event::<()>(
            imgui.context.io_mut(),
            &window.window,
            &Event::UserEvent(event),
        );
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();
        imgui.platform.handle_event::<()>(
            imgui.context.io_mut(),
            &window.window,
            &Event::DeviceEvent { device_id, event },
        );
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();

        window.window.request_redraw();
        imgui.platform.handle_event::<()>(
            imgui.context.io_mut(),
            &window.window,
            &Event::AboutToWait,
        );
    }
}

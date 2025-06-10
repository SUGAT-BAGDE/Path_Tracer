use std::time::Instant;

use imgui::{TextureId};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use wgpu::wgt::TextureViewDescriptor;

use crate::ui::app_window::AppWindow;
use crate::ui::utils::create_texture_from_pixels;

#[derive(Default)]
pub struct App {
    window: Option<AppWindow>
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

                window.surface.configure(&window.device, &window.surface_desc);
            },
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                let delta_s = imgui.last_frame.elapsed();
                let now = Instant::now();
                
                imgui.context
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

                imgui.platform
                    .prepare_frame(imgui.context.io_mut(), &window.window)
                    .expect("Failed to prepare frame");

                let ui = imgui.context.frame();
                let mut frame_id : Option<TextureId> = None ;
                
                // Lets create a window
                {
                    // write docking mechanism
                    ui.dockspace_over_main_viewport();

                    let imgui_window = ui.window("This is my Window");
                    imgui_window.size([300.0, 200.0], imgui::Condition::FirstUseEver)
                        .position([200.0, 400.0], imgui::Condition::FirstUseEver)
                        .build(|| {
                            ui.text(format!("delta_s : {delta_s:?}"));
                            ui.button("Render").then(||{
                                println!("Clicked Render!!");
                            });
                        });

                    let style_guard = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));

                    let imgui_window = ui.window("Viewport");
                    imgui_window
                        .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                        .build(|| {
                            let size = ui.content_region_avail();
                            let width = size[0] as u32;
                            let height = size[1] as u32;

                            let magenta = 0xFF464646_u32; // ARGB or BGRA black with full alpha
                            let pixels = vec![magenta; (width * height) as usize];

                            let texture_id = create_texture_from_pixels(pixels.as_slice(), width, height, &window.device, &window.queue, &mut imgui.renderer);

                            imgui::Image::new(
                                texture_id.clone(), 
                                size
                            )
                            .build(&ui);
                            
                            frame_id = Some(texture_id);
                        });
                    
                    drop(style_guard);                    
                }

                let mut encoder = window.device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {label : None});

                if imgui.last_cursor != ui.mouse_cursor() {
                    imgui.last_cursor = ui.mouse_cursor();
                    imgui.platform.prepare_render(&ui, &window.window);
                }

                let view = frame.texture
                    .create_view(&TextureViewDescriptor::default());

                let mut rpass = encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor { 
                        label: None, 
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations { 
                                load: wgpu::LoadOp::Clear(imgui.clear_color), 
                                store: wgpu::StoreOp::Store
                            }
                        })], 
                        depth_stencil_attachment: None, 
                        timestamp_writes: None, 
                        occlusion_query_set: None 
                    });
                
                imgui.renderer.render(
                    imgui.context.render(), 
                    &window.queue, 
                    &window.device, 
                    &mut rpass
                ).expect("Rendering Failed!...");

                drop(rpass);

                imgui.renderer.textures.remove(frame_id.unwrap());

                window.queue.submit(Some(encoder.finish()));
                frame.present();
            },
            _ => ()
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
        imgui.platform.handle_event::<()>(imgui.context.io_mut(), &window.window, &Event::AboutToWait);
    }    
}

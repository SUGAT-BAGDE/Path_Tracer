use std::sync::Arc;
use std::time::Instant;

use bytemuck;
use pollster::block_on;

use imgui::MouseCursor;
use imgui_wgpu::RendererConfig;
use imgui_wgpu::Renderer;
use imgui_winit_support::WinitPlatform;

use wgpu::Device;
use wgpu::Extent3d;
use wgpu::Queue;

use winit::event_loop::{EventLoop, ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowId};
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};

struct ImGuiState {
    context : imgui::Context,
    platform : WinitPlatform,
    renderer : Renderer,
    last_frame : Instant,
    clear_color : wgpu::Color,
    last_cursor : Option<MouseCursor>
}

struct MyWindow {
    window : Arc<Window>,
    device : wgpu::Device,
    queue : wgpu::Queue,
    surface_desc : wgpu::SurfaceConfiguration,
    surface : wgpu::Surface<'static>,
    hidpi_factor : f64,
    imgui : Option<ImGuiState>
}

#[derive(Default)]
struct App {
    window: Option<MyWindow>
}


impl MyWindow {
    fn setup_gpu(event_loop: &ActiveEventLoop) -> Self {
        let instance = wgpu::Instance::default();

        let window = {
            let version = env!("CARGO_PKG_VERSION");

            let attributes = Window::default_attributes()
                .with_title(format!("Sugat Path Tracer {version}"));
            Arc::new(event_loop.create_window(attributes).unwrap())
        };

        let size = window.inner_size();
        let hidpi_factor = window.scale_factor();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase { 
            power_preference: wgpu::PowerPreference::HighPerformance, 
            force_fallback_adapter:  false,
            compatible_surface: Some(&surface) }
        )).unwrap();

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
            .unwrap();

        // Set up swap chain
        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
        };

        surface.configure(&device, &surface_desc);

        let imgui = None;

        return Self { window, imgui, device, queue, surface_desc, hidpi_factor, surface };
    }

    fn setup_imgui(&mut self)
    {
        let mut context = imgui::Context::create();
        
        context.set_ini_filename(None);
        context.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
        
        let mut platform = imgui_winit_support::WinitPlatform::new(&mut context);
        platform.attach_window(
            context.io_mut(), 
            &self.window, 
            imgui_winit_support::HiDpiMode::Default
        );

        let _font_size = (13.0 * self.hidpi_factor) as f32;
        context.io_mut().font_global_scale = (1.0 / self.hidpi_factor) as f32;

        // context.fonts().add_font(&[FontSource::DefaultFontData {
        //     config: Some(imgui::FontConfig {
        //         oversample_h: 1,
        //         pixel_snap_h: true,
        //         size_pixels: font_size,
        //         ..Default::default()
        //     }),
        // }]);

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };


        let renderer_config = RendererConfig {
            texture_format : self.surface_desc.format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut context, &self.device, &self.queue, renderer_config);

        let last_cursor = None;
        let last_frame = Instant::now();

        self.imgui = Some(ImGuiState{
            renderer,
            context,
            platform,
            last_cursor,
            last_frame,
            clear_color
        });
    }

    fn new(event_loop: &ActiveEventLoop) -> Self {
        let mut window = Self::setup_gpu(event_loop);
        window.setup_imgui();

        return  window;
    }

}

impl Drop for MyWindow {
    fn drop(&mut self) {
        self.imgui = None; // drop ImGui first
    }
}


fn create_texture_from_pixels(pixels: &[u32], width:u32, height:u32, device : &Device, queue : &Queue, renderer: &mut imgui_wgpu::Renderer) -> imgui::TextureId {
    let size = Extent3d {
        width,
        height,
        ..Default::default()
    };

    let pixel_bytes = bytemuck::cast_slice(pixels);

    let config  = imgui_wgpu::TextureConfig{
        size,
        label : Some("Imgui Texture"),
        format: Some(wgpu::TextureFormat::Bgra8UnormSrgb),
        ..Default::default()
    };

    let texture = imgui_wgpu::Texture::new(device, renderer, config);
    texture.write(queue, pixel_bytes, width, height);

    renderer.textures.insert(texture)
        
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(MyWindow::new(event_loop))
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
                window.surface_desc = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::Fifo,
                    desired_maximum_frame_latency: 2,
                    alpha_mode: wgpu::CompositeAlphaMode::Auto,
                    view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
                };

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
                
                // Lets create a window
                {

                    // write docking mechanism
                    ui.dockspace_over_main_viewport();


                    let imgui_window = ui.window("This is my Window");
                    imgui_window.size([300.0, 200.0], imgui::Condition::FirstUseEver)
                        .position([200.0, 400.0], imgui::Condition::FirstUseEver)
                        .build(|| {
                            ui.text(format!("delta_s : {delta_s:?}"));
                            ui.button("Render");
                        });

                    let style_guard = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));

                    let imgui_window = ui.window("Viewport");
                    imgui_window
                        .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                        .build(|| {
                            let size = ui.content_region_avail();
                            let width = size[0] as u32;
                            let height = size[1] as u32;
                            // let height = window_size;
                            // ui.text(format!("Window height: {:.1}", height)); 
                            // ui.text(format!("Window width: {:.1}", width)); 

                            let magenta = 0xFFFF00FF_u32; // ARGB or BGRA black with full alpha
                            let pixels = vec![magenta; (width * height) as usize];

                            imgui::Image::new(
                                create_texture_from_pixels(pixels.as_slice(), width, height, &window.device, &window.queue, &mut imgui.renderer), 
                                size
                            )
                            .build(&ui);
                            
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
                    .create_view(&wgpu::wgt::TextureViewDescriptor::default());

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

fn main() {

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);

}

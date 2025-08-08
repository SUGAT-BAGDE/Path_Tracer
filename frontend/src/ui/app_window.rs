use std::sync::Arc;

use std::time::Instant;

use pollster::block_on;

use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;

use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use crate::constants::ALPHA_MODE;
use crate::constants::DISPLAY_TEXTURE_FORMAT;
use crate::constants::PRESENT_MODE;
use crate::constants::VIEW_FORMATS;
use crate::ui::imgui_state::ImGuiState;

pub struct AppWindow {
    pub window: Arc<Window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_desc: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub hidpi_factor: f64,
    pub imgui: Option<ImGuiState>,
}

impl AppWindow {
    fn setup_gpu(event_loop: &ActiveEventLoop) -> Self {
        let instance = wgpu::Instance::default();

        let window = {
            let version = env!("CARGO_PKG_VERSION");

            let attributes =
                Window::default_attributes().with_title(format!("Sugat Path Tracer {version}"));
            Arc::new(event_loop.create_window(attributes).unwrap())
        };

        let size = window.inner_size();
        let hidpi_factor = window.scale_factor();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

        // Set up swap chain
        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: DISPLAY_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            present_mode: PRESENT_MODE,
            desired_maximum_frame_latency: 2,
            alpha_mode: ALPHA_MODE,
            view_formats: VIEW_FORMATS.to_vec(),
        };

        surface.configure(&device, &surface_desc);

        let imgui = None;

        Self {
            window,
            imgui,
            device,
            queue,
            surface_desc,
            hidpi_factor,
            surface,
        }
    }

    fn setup_imgui(&mut self) {
        let mut context = imgui::Context::create();

        context.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

        context.set_ini_filename(None);

        let mut platform = imgui_winit_support::WinitPlatform::new(&mut context);
        platform.attach_window(
            context.io_mut(),
            &self.window,
            imgui_winit_support::HiDpiMode::Default,
        );

        let _font_size = (13.0 * self.hidpi_factor) as f32;
        context.io_mut().font_global_scale = (1.0 / self.hidpi_factor) as f32;

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let renderer_config = RendererConfig {
            texture_format: self.surface_desc.format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut context, &self.device, &self.queue, renderer_config);

        let last_cursor = None;
        let last_frame = Instant::now();

        self.imgui = Some(ImGuiState {
            renderer,
            context,
            platform,
            last_cursor,
            last_frame,
            clear_color,
            current_frame : None
        });
    }

    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let mut window = Self::setup_gpu(event_loop);
        window.setup_imgui();

        window
    }
}

impl Drop for AppWindow {
    fn drop(&mut self) {
        self.imgui = None; // drop ImGui first
    }
}

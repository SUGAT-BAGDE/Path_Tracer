use imgui::{MouseCursor, Ui};
use imgui_wgpu::Renderer;
use imgui_winit_support::WinitPlatform;
use std::time::Instant;

pub struct ImGuiState {
    pub context: imgui::Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub last_frame: Instant,
    pub clear_color: wgpu::Color,
    pub last_cursor: Option<MouseCursor>,
    
    pub current_frame : Option<Ui>,
}

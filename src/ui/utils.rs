use imgui::TextureId;
use imgui_wgpu::Renderer;
use wgpu::{Device, Extent3d, Queue};

use crate::constants::DISPLAY_TEXTURE_FORMAT;

pub fn create_texture_from_pixels(
    pixels: &[u32],
    dimensions: [u32; 2],
    device: &Device,
    queue: &Queue,
    renderer: &mut Renderer,
) -> TextureId {
    let size = Extent3d {
        width: dimensions[0],
        height: dimensions[1],
        ..Default::default()
    };

    let pixel_bytes = bytemuck::cast_slice(pixels);

    let config = imgui_wgpu::TextureConfig {
        size,
        label: Some("Imgui Texture"),
        format: Some(DISPLAY_TEXTURE_FORMAT),
        ..Default::default()
    };

    let texture = imgui_wgpu::Texture::new(device, renderer, config);
    texture.write(queue, pixel_bytes, dimensions[0], dimensions[1]);

    renderer.textures.insert(texture)
}

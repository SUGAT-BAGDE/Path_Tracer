use wgpu::{Device, Extent3d, Queue};
use imgui::TextureId;
use imgui_wgpu::Renderer;


pub fn create_texture_from_pixels(pixels: &[u32], width:u32, height:u32, device : &Device, queue : &Queue, renderer: &mut Renderer) -> TextureId {
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

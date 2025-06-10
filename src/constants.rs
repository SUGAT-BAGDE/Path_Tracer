use wgpu::{CompositeAlphaMode, PresentMode, TextureFormat};

pub const DISPLAY_TEXTURE_FORMAT : TextureFormat = TextureFormat:: Bgra8UnormSrgb;
pub const VIEW_FORMATS : &[TextureFormat] = &[TextureFormat::Bgra8Unorm];
pub const ALPHA_MODE : CompositeAlphaMode = CompositeAlphaMode::Auto;
pub const PRESENT_MODE : PresentMode = PresentMode::Fifo;
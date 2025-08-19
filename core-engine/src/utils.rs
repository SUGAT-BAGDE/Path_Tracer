use glam::Vec4;

/// Converts 0.0 to 1.0 to u32 in format 0xAARRGGBB
pub(super) fn convert_to_argb(color: &Vec4) -> u32 /* Results Color */ {
    (((color.w * 255.0) as u32) << 24)
        | (((color.x * 255.0) as u32) << 16)
        | (((color.y * 255.0) as u32) << 8)
        | ((color.z * 255.0) as u32)
}

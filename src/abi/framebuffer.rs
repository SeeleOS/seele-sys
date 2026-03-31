#[repr(C)]
pub struct FramebufferInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bytes_per_pixel: u32,
    pub byte_len: u64,
    pub pixel_format: u32,
}

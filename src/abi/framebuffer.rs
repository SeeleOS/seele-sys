#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FramebufferPixelFormat {
    #[default]
    Rgb = 0,
    Bgr = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FramebufferInfo {
    pub phys_addr: usize,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub bytes_per_pixel: usize,
    pub byte_len: usize,
    pub pixel_format: FramebufferPixelFormat,
}

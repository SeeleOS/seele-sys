#[repr(C)]
pub struct SystemInfo {
    name: [u8; 64],
    version: [u8; 64],
}

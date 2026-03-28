#[repr(C)]
pub struct SystemInfo {
    name: [u8; 64],
    version: [u8; 64],
}

impl SystemInfo {
    pub fn new(name: &str, version: &str) -> Self {
        let mut info = Self {
            name: [0; 64],
            version: [0; 64],
        };

        write_c_string(&mut info.name, name);
        write_c_string(&mut info.version, version);

        info
    }

    pub fn name(&self) -> &[u8; 64] {
        &self.name
    }

    pub fn version(&self) -> &[u8; 64] {
        &self.version
    }
}

fn write_c_string(dst: &mut [u8], src: &str) {
    let bytes = src.as_bytes();
    let len = bytes.len().min(dst.len().saturating_sub(1));
    dst[..len].copy_from_slice(&bytes[..len]);
}

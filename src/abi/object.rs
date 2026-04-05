use num_enum::TryFromPrimitive;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TerminalInfo {
    pub rows: u64,
    pub cols: u64,
    pub echo: bool,
    /// Whether the kernel tty applies canonical line discipline semantics.
    pub canonical: bool,
    pub echo_newline: bool,
    pub echo_delete: bool,
}

impl TerminalInfo {
    pub const fn new(rows: u64, cols: u64) -> Self {
        Self {
            rows,
            cols,
            echo: true,
            canonical: true,
            echo_newline: true,
            echo_delete: true,
        }
    }
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum ConfigCommand {
    GetTerminalInfo = 0,
    SetTerminalInfo = 1,
    GetFramebufferInfo = 2,
    FbTakeControl = 3,
    FbRelease = 4,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ObjectFlags: u64 {
        const NONBLOCK = 1 << 0;
    }
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum ControlCommand {
    SetFlags = 0,
    GetFlags = 1,
    CloneWithMin = 2,
    CloneWithMinCloseOnExecve = 3,
    SetObjectFlags = 4,
    GetObjectFlags = 5,
}

impl ControlCommand {
    pub fn from_linux(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::GetObjectFlags),
            2 => Some(Self::SetObjectFlags),
            3 => Some(Self::GetFlags),
            4 => Some(Self::SetFlags),
            0 => Some(Self::CloneWithMin),
            1030 => Some(Self::CloneWithMinCloseOnExecve),
            _ => None,
        }
    }
}

pub fn device_from_path(path: &str) -> Option<*const u8> {
    match path {
        "/dev/fb0" => Some(c"framebuffer".as_ptr().cast()),
        "/dev/null" => Some(c"devnull".as_ptr().cast()),
        _ => None,
    }
}

#[repr(u64)]
#[derive(Clone, Copy, TryFromPrimitive, Debug)]
pub enum SeekType {
    Start = 0,
    Current = 1,
    End = 2,
}

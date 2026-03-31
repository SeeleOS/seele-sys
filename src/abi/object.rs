use crate::{SyscallResult, errors::SyscallError};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigCommand {
    GetTerminalInfo = 0,
    SetTerminalInfo = 1,
    GetFramebufferInfo = 2,
    FbTakeControl = 3,
    FbRelease = 4,
}

impl ConfigCommand {
    pub fn from_raw_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(Self::GetTerminalInfo),
            1 => Some(Self::SetTerminalInfo),
            _ => None,
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ObjectFlags: u64 {
        const NONBLOCK = 1 << 0;
    }
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlCommand {
    SetFlags = 0,
    GetFlags = 1,
}

impl ControlCommand {
    pub fn from_u64(value: u64) -> SyscallResult<Self> {
        match value {
            0 => Ok(Self::SetFlags),
            1 => Ok(Self::GetFlags),
            _ => Err(SyscallError::InvalidArguments),
        }
    }

    pub fn from_linux(value: i32) -> Option<Self> {
        match value {
            3 => Some(Self::GetFlags),
            4 => Some(Self::SetFlags),
            _ => None,
        }
    }
}

use crate::{syscall, utils::SyscallResult};

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

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlCommand {
    SetFlags = 0,
    GetFlags = 1,
}

impl ControlCommand {
    pub fn from_raw_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(Self::SetFlags),
            1 => Some(Self::GetFlags),
            _ => None,
        }
    }

    pub fn from_linux(value: i32) -> Option<Self> {
        Self::from_raw_u64(value as u64)
    }

    pub fn from_linux_u64(value: u64) -> Option<Self> {
        Self::from_raw_u64(value)
    }
}

pub fn read_object(object: u64, buffer: &mut [u8]) -> SyscallResult {
    syscall!(
        ReadObject,
        object,
        buffer.as_ptr() as u64,
        buffer.len() as u64
    )
}

pub fn write_object(object: u64, buffer: &[u8]) -> SyscallResult {
    syscall!(
        WriteObject,
        object,
        buffer.as_ptr() as u64,
        buffer.len() as u64
    )
}

pub fn configurate_object(object: u64, request_num: u64, ptr: *mut u8) -> SyscallResult {
    syscall!(ConfigurateObject, object, request_num, ptr as u64)
}

pub fn get_terminal_info(object: u64, info: *mut TerminalInfo) -> SyscallResult {
    configurate_object(object, ConfigCommand::GetTerminalInfo as u64, info.cast())
}

pub fn set_terminal_info(object: u64, info: *const TerminalInfo) -> SyscallResult {
    configurate_object(
        object,
        ConfigCommand::SetTerminalInfo as u64,
        info.cast_mut().cast(),
    )
}

pub fn control_object(object: u64, command: ControlCommand, arg: u64) -> SyscallResult {
    syscall!(ControlObject, object, command as u64, arg)
}

pub fn remove_object(object: u64) -> SyscallResult {
    syscall!(RemoveObject, object)
}

pub fn clone_object(object: u64) -> SyscallResult {
    syscall!(CloneObject, object)
}

pub fn clone_object_to(object: u64, dest: u64) -> SyscallResult {
    syscall!(CloneObjectTo, object, dest)
}

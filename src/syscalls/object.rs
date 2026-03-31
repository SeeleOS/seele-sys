use crate::{
    SyscallResult,
    abi::object::{ConfigCommand, ControlCommand, TerminalInfo},
    permission::Permissions,
    syscall,
};

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

pub fn open_device(name: *mut u8) -> SyscallResult {
    syscall!(OpenDevice, name as u64)
}

pub fn mmap_object(
    object: u64,
    pages: u64,
    offset: u64,
    permissions: Permissions,
) -> SyscallResult {
    syscall!(MmapObject, object, pages, offset, permissions.bits())
}

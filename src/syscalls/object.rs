use crate::{
    SyscallResult,
    abi::{
        framebuffer::FramebufferInfo,
        object::{ConfigCommand, ControlCommand, SeekType, TerminalInfo},
    },
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

pub fn get_framebuffer_info(object: u64, info: *mut FramebufferInfo) -> SyscallResult {
    configurate_object(
        object,
        ConfigCommand::GetFramebufferInfo as u64,
        info.cast(),
    )
}

pub fn framebuffer_take_control(object: u64) -> SyscallResult {
    configurate_object(
        object,
        ConfigCommand::FbTakeControl as u64,
        core::ptr::null_mut(),
    )
}

pub fn framebuffer_release(object: u64) -> SyscallResult {
    configurate_object(
        object,
        ConfigCommand::FbRelease as u64,
        core::ptr::null_mut(),
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

pub fn open_device(name: *const u8) -> SyscallResult {
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

pub fn seek_object(object: u64, offset: i64, seek_type: SeekType) -> SyscallResult {
    syscall!(SeekObject, object, offset as u64, seek_type as u64)
}

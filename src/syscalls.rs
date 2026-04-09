use core::ffi::c_char;

use crate::{SyscallResult, misc::SystemInfo, permission::Permissions, syscall};

pub mod filesystem;
pub mod futex;
pub mod misc;
pub mod object;
pub mod polling;
pub mod signal;
pub mod socket;
pub mod timer;

// Wait for a process to exit
pub fn wait_for_process_exit(process: i32, exit_code_ptr: *mut i32) -> SyscallResult {
    syscall!(WaitForProcessExit, process as u64, exit_code_ptr as u64)
}

pub fn exit(code: u64) -> SyscallResult {
    syscall!(Exit, code)
}

pub fn fork() -> SyscallResult {
    syscall!(Fork)
}

pub fn set_fs(addr: u64) -> SyscallResult {
    syscall!(SetFs, addr)
}

pub fn get_fs() -> SyscallResult {
    syscall!(GetFs)
}

pub fn set_gs(addr: u64) -> SyscallResult {
    syscall!(SetGs, addr)
}

fn allocate_mem_pages(pages: u64, flags: u64, permissions: Permissions) -> SyscallResult {
    syscall!(AllocateMem, pages, flags, permissions.bits())
}

fn update_mem_perms_by_pages(addr: u64, pages: u64, permissions: Permissions) -> SyscallResult {
    syscall!(UpdateMemPerms, addr, pages, permissions.bits())
}

pub fn update_mem_perms(addr: u64, len: u64, permissions: Permissions) -> SyscallResult {
    update_mem_perms_by_pages(addr, len.div_ceil(4096), permissions)
}

pub fn allocate_mem(len: u64, flags: u64, permissions: Permissions) -> SyscallResult {
    allocate_mem_pages(len.div_ceil(4096), flags, permissions)
}

pub fn get_process_id() -> SyscallResult {
    syscall!(GetProcessID)
}

pub fn get_process_parent_id() -> SyscallResult {
    syscall!(GetProcessParentID)
}

pub fn get_thread_id() -> SyscallResult {
    // TODO not yet implemented
    get_process_id()
}

pub fn execve(path: &str, args: *const *mut c_char, env: *const *mut c_char) -> SyscallResult {
    syscall!(
        Execve,
        path.as_bytes().as_ptr() as u64,
        args as u64,
        env as u64
    )
}

pub fn deallocate_mem(addr: u64, len: u64) -> SyscallResult {
    syscall!(DeallocateMem, addr, len)
}

pub fn get_system_info(info: *mut SystemInfo) -> SyscallResult {
    syscall!(GetSystemInfo, info as u64)
}

use crate::{SyscallResult, syscall};

pub fn get_current_time() -> SyscallResult {
    syscall!(GetCurrentTime)
}

pub fn time_since_boot() -> SyscallResult {
    syscall!(TimeSinceBoot)
}

pub fn sleep(nanoseconds: u64) -> SyscallResult {
    syscall!(Sleep, nanoseconds)
}

pub fn thread_clone(stack_pointer: u64) -> SyscallResult {
    syscall!(ThreadClone, stack_pointer)
}

pub fn set_process_group_id(process: u64, group_id: u64) -> SyscallResult {
    syscall!(SetProcessGroupID, process, group_id)
}

pub fn get_process_group_id(process: u64) -> SyscallResult {
    syscall!(GetProcessGroupID, process)
}

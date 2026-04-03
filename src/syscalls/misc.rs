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

use crate::{syscall, utils::SyscallResult};

pub fn get_current_time() -> SyscallResult {
    syscall!(GetCurrentTime)
}

pub fn time_since_boot() -> SyscallResult {
    syscall!(TimeSinceBoot)
}

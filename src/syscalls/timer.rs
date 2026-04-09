use crate::{
    SyscallResult,
    abi::time::{TimeType, TimerNotifyStruct, TimerStateStruct},
    syscall,
};

/// Returns the timer id
pub fn create_timer(time_type: TimeType, notify_type: *const TimerNotifyStruct) -> SyscallResult {
    syscall!(CreateTimer, time_type as u64, notify_type as u64)
}

pub fn delete_timer(id: u64) -> SyscallResult {
    syscall!(DeleteTimer, id)
}

pub fn set_timer_state(id: u64, timer_state: *const TimerStateStruct) -> SyscallResult {
    syscall!(SetTimerState, id, timer_state as u64)
}

pub fn get_timer_state(id: u64, timer_state: *mut TimerStateStruct) -> SyscallResult {
    syscall!(GetTimerState, id, timer_state as u64)
}

pub fn get_timer_overrun(id: u64) -> SyscallResult {
    syscall!(GetTimerOverrun, id)
}

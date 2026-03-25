use crate::{
    signal::{Signal, SignalAction},
    syscall,
    utils::SyscallResult,
};

pub fn send_signal(process: u64, signal: Signal) -> SyscallResult {
    syscall!(SendSignal, process, signal as u64)
}

pub fn register_signal_action(
    signal: Signal,
    new_action: *const SignalAction,
    old_action: *mut SignalAction,
) -> SyscallResult {
    syscall!(
        RegisterSignalAction,
        signal as u64,
        new_action as u64,
        old_action as u64
    )
}

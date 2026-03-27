use crate::{
    signal::{Signal, SignalAction, Signals},
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

pub fn block_signals(signals: Signals, old_signals: *mut Signals) -> SyscallResult {
    syscall!(BlockSignals, signals.bits(), old_signals as u64)
}

pub fn unblock_signals(signals: Signals, old_signals: *mut Signals) -> SyscallResult {
    syscall!(UnblockSignals, signals.bits(), old_signals as u64)
}

pub fn set_blocked_signals(signals: Signals, old_signals: *mut Signals) -> SyscallResult {
    syscall!(SetBlockedSignals, signals.bits(), old_signals as u64)
}

pub fn sig_handler_return() -> SyscallResult {
    syscall!(SigHandlerReturn)
}

use bitflags::bitflags;
use core::ffi::c_void;
use num_enum::TryFromPrimitive;
use strum::EnumIter;

#[derive(Clone, Copy, TryFromPrimitive, Debug, EnumIter)]
#[repr(u64)]
pub enum Signal {
    Terminate = 0,
    Kill,
    Interrupt,
    Quit,
    Abort,
    InvalidMemoryAccess,
    ChildChanged,
    Stop,
    Continue,
    BrokenPipe,
    Alarm,
    Hangup,
    TerminalStop,
    FloatingPointError,
    IllegalInstruction,
    Trap,
}

pub const SIGNAL_AMOUNT: usize = 16;

pub type SignalHandlerFn = extern "C" fn(i32);
pub type SignalActionFn = extern "C" fn(i32, *const SigInfo, *const UContext);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SigInfo {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub si_pid: i32,
    pub si_uid: u32,
    pub si_addr: *mut c_void,
    pub si_status: i32,
    pub si_value: SigValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union SigValue {
    pub sival_int: i32,
    pub sival_ptr: *mut c_void,
}

impl Default for SigValue {
    fn default() -> Self {
        Self { sival_int: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UContext {
    pub blocked_signals: u64,
    pub gregs: [u64; 20],
}

/// The action that a process will take when it got a signal.
#[derive(Default, Clone, Debug)]
#[repr(C)]
pub struct SignalAction {
    pub handling_type: SignalHandlingType,
    // Signals which the process will ignore when its in the signal handler.
    pub sig_handler_ignored_sigs: Signals,
    pub flags: u64,
    // User-space restorer the kernel uses as the handler's return address.
    // The restorer should call sigreturn to return.
    pub restorer: usize,
}

#[derive(Default, Clone, Debug)]
#[repr(C)]
pub enum SignalHandlingType {
    #[default]
    Default,
    Ignore,
    Function1(SignalHandlerFn),
    Function2(SignalActionFn),
}

bitflags! {
    #[derive(Default, Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Signals: u64 {
        const TERMINATE = 1 << Signal::Terminate as u64;
        const KILL = 1 << Signal::Kill as u64;
        const INTERRUPT = 1 << Signal::Interrupt as u64;
        const QUIT = 1 << Signal::Quit as u64;
        const ABORT = 1 << Signal::Abort as u64;
        const INVALID_MEMORY_ACCESS = 1 << Signal::InvalidMemoryAccess as u64;
        const CHILD_CHANGED = 1 << Signal::ChildChanged as u64;
        const STOP = 1 << Signal::Stop as u64;
        const CONTINUE = 1 << Signal::Continue as u64;
        const BROKEN_PIPE = 1 << Signal::BrokenPipe as u64;
        const ALARM = 1 << Signal::Alarm as u64;
        const HANGUP = 1 << Signal::Hangup as u64;
        const TERMINAL_STOP = 1 << Signal::TerminalStop as u64;
        const FLOATING_POINT_ERROR = 1 << Signal::FloatingPointError as u64;
        const ILLEGAL_INSTRUCTION = 1 << Signal::IllegalInstruction as u64;
        const TRAP = 1 << Signal::Trap as u64;
    }
}

impl From<Signal> for Signals {
    fn from(value: Signal) -> Self {
        match value {
            Signal::Terminate => Self::TERMINATE,
            Signal::Kill => Self::KILL,
            Signal::Interrupt => Self::INTERRUPT,
            Signal::Quit => Self::QUIT,
            Signal::Abort => Self::ABORT,
            Signal::InvalidMemoryAccess => Self::INVALID_MEMORY_ACCESS,
            Signal::ChildChanged => Self::CHILD_CHANGED,
            Signal::Stop => Self::STOP,
            Signal::Continue => Self::CONTINUE,
            Signal::BrokenPipe => Self::BROKEN_PIPE,
            Signal::Alarm => Self::ALARM,
            Signal::Hangup => Self::HANGUP,
            Signal::TerminalStop => Self::TERMINAL_STOP,
            Signal::FloatingPointError => Self::FLOATING_POINT_ERROR,
            Signal::IllegalInstruction => Self::ILLEGAL_INSTRUCTION,
            Signal::Trap => Self::TRAP,
        }
    }
}

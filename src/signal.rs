use bitflags::bitflags;
use core::ffi::c_void;
use num_enum::TryFromPrimitive;
use strum::EnumIter;

#[derive(Clone, Copy, TryFromPrimitive, Debug, EnumIter)]
#[repr(u64)]
pub enum Signal {
    Hangup = 1,
    Interrupt = 2,
    Quit = 3,
    IllegalInstruction = 4,
    Trap = 5,
    Abort = 6,
    BusError = 7,
    FloatingPointError = 8,
    Kill = 9,
    User1 = 10,
    InvalidMemoryAccess = 11,
    User2 = 12,
    BrokenPipe = 13,
    Alarm = 14,
    Terminate = 15,
    ChildChanged = 17,
    Continue = 18,
    Stop = 19,
    TerminalStop = 20,
    CpuTimeLimitExceeded = 24,
    FileSizeLimitExceeded = 25,
    BadSystemCall = 31,
}

pub const SIGNAL_AMOUNT: usize = 22;

pub type SignalHandlerFn = extern "C" fn(i32);
pub type SigHandlerFn2 = extern "C" fn(i32, *const SigInfo, *const UContext);

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
    Function2(SigHandlerFn2),
}

impl Signal {
    pub const fn index(self) -> usize {
        match self {
            Self::Hangup => 0,
            Self::Interrupt => 1,
            Self::Quit => 2,
            Self::IllegalInstruction => 3,
            Self::Trap => 4,
            Self::Abort => 5,
            Self::BusError => 6,
            Self::FloatingPointError => 7,
            Self::Kill => 8,
            Self::User1 => 9,
            Self::InvalidMemoryAccess => 10,
            Self::User2 => 11,
            Self::BrokenPipe => 12,
            Self::Alarm => 13,
            Self::Terminate => 14,
            Self::ChildChanged => 15,
            Self::Continue => 16,
            Self::Stop => 17,
            Self::TerminalStop => 18,
            Self::CpuTimeLimitExceeded => 19,
            Self::FileSizeLimitExceeded => 20,
            Self::BadSystemCall => 21,
        }
    }
}

bitflags! {
    #[derive(Default, Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Signals: u64 {
        const HANGUP = 1 << Signal::Hangup as u64;
        const INTERRUPT = 1 << Signal::Interrupt as u64;
        const QUIT = 1 << Signal::Quit as u64;
        const ILLEGAL_INSTRUCTION = 1 << Signal::IllegalInstruction as u64;
        const TRAP = 1 << Signal::Trap as u64;
        const ABORT = 1 << Signal::Abort as u64;
        const BUS_ERROR = 1 << Signal::BusError as u64;
        const FLOATING_POINT_ERROR = 1 << Signal::FloatingPointError as u64;
        const KILL = 1 << Signal::Kill as u64;
        const USER1 = 1 << Signal::User1 as u64;
        const INVALID_MEMORY_ACCESS = 1 << Signal::InvalidMemoryAccess as u64;
        const USER2 = 1 << Signal::User2 as u64;
        const BROKEN_PIPE = 1 << Signal::BrokenPipe as u64;
        const ALARM = 1 << Signal::Alarm as u64;
        const TERMINATE = 1 << Signal::Terminate as u64;
        const CHILD_CHANGED = 1 << Signal::ChildChanged as u64;
        const CONTINUE = 1 << Signal::Continue as u64;
        const STOP = 1 << Signal::Stop as u64;
        const TERMINAL_STOP = 1 << Signal::TerminalStop as u64;
        const CPU_TIME_LIMIT_EXCEEDED = 1 << Signal::CpuTimeLimitExceeded as u64;
        const FILE_SIZE_LIMIT_EXCEEDED = 1 << Signal::FileSizeLimitExceeded as u64;
        const BAD_SYSTEM_CALL = 1 << Signal::BadSystemCall as u64;
    }
}

impl From<Signal> for Signals {
    fn from(value: Signal) -> Self {
        match value {
            Signal::Hangup => Self::HANGUP,
            Signal::Interrupt => Self::INTERRUPT,
            Signal::Quit => Self::QUIT,
            Signal::IllegalInstruction => Self::ILLEGAL_INSTRUCTION,
            Signal::Trap => Self::TRAP,
            Signal::Abort => Self::ABORT,
            Signal::BusError => Self::BUS_ERROR,
            Signal::FloatingPointError => Self::FLOATING_POINT_ERROR,
            Signal::Kill => Self::KILL,
            Signal::User1 => Self::USER1,
            Signal::InvalidMemoryAccess => Self::INVALID_MEMORY_ACCESS,
            Signal::User2 => Self::USER2,
            Signal::BrokenPipe => Self::BROKEN_PIPE,
            Signal::Alarm => Self::ALARM,
            Signal::Terminate => Self::TERMINATE,
            Signal::ChildChanged => Self::CHILD_CHANGED,
            Signal::Continue => Self::CONTINUE,
            Signal::Stop => Self::STOP,
            Signal::TerminalStop => Self::TERMINAL_STOP,
            Signal::CpuTimeLimitExceeded => Self::CPU_TIME_LIMIT_EXCEEDED,
            Signal::FileSizeLimitExceeded => Self::FILE_SIZE_LIMIT_EXCEEDED,
            Signal::BadSystemCall => Self::BAD_SYSTEM_CALL,
        }
    }
}

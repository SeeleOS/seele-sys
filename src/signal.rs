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
    TerminalInput = 21,
    TerminalOutput = 22,
    CpuTimeLimitExceeded = 24,
    FileSizeLimitExceeded = 25,
    BadSystemCall = 31,
}

pub const SIGNAL_AMOUNT: usize = 24;

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
            Self::TerminalInput => 19,
            Self::TerminalOutput => 20,
            Self::CpuTimeLimitExceeded => 21,
            Self::FileSizeLimitExceeded => 22,
            Self::BadSystemCall => 23,
        }
    }

    pub const fn mask(self) -> u64 { 1 << (self as u64 - 1) }
}

bitflags! {
    #[derive(Default, Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Signals: u64 {
        const HANGUP = Signal::Hangup.mask();
        const INTERRUPT = Signal::Interrupt.mask();
        const QUIT = Signal::Quit.mask();
        const ILLEGAL_INSTRUCTION = Signal::IllegalInstruction.mask();
        const TRAP = Signal::Trap.mask();
        const ABORT = Signal::Abort.mask();
        const BUS_ERROR = Signal::BusError.mask();
        const FLOATING_POINT_ERROR = Signal::FloatingPointError.mask();
        const KILL = Signal::Kill.mask();
        const USER1 = Signal::User1.mask();
        const INVALID_MEMORY_ACCESS = Signal::InvalidMemoryAccess.mask();
        const USER2 = Signal::User2.mask();
        const BROKEN_PIPE = Signal::BrokenPipe.mask();
        const ALARM = Signal::Alarm.mask();
        const TERMINATE = Signal::Terminate.mask();
        const CHILD_CHANGED = Signal::ChildChanged.mask();
        const CONTINUE = Signal::Continue.mask();
        const STOP = Signal::Stop.mask();
        const TERMINAL_STOP = Signal::TerminalStop.mask();
        const TERMINAL_INPUT = Signal::TerminalInput.mask();
        const TERMINAL_OUTPUT = Signal::TerminalOutput.mask();
        const CPU_TIME_LIMIT_EXCEEDED = Signal::CpuTimeLimitExceeded.mask();
        const FILE_SIZE_LIMIT_EXCEEDED = Signal::FileSizeLimitExceeded.mask();
        const BAD_SYSTEM_CALL = Signal::BadSystemCall.mask();
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
            Signal::TerminalInput => Self::TERMINAL_INPUT,
            Signal::TerminalOutput => Self::TERMINAL_OUTPUT,
            Signal::CpuTimeLimitExceeded => Self::CPU_TIME_LIMIT_EXCEEDED,
            Signal::FileSizeLimitExceeded => Self::FILE_SIZE_LIMIT_EXCEEDED,
            Signal::BadSystemCall => Self::BAD_SYSTEM_CALL,
        }
    }
}

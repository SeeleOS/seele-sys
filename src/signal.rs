use bitflags::bitflags;
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

/// The action that a process will take when it got a signal.
#[derive(Default, Clone, Debug)]
#[repr(C)]
pub struct SignalAction {
    pub handling_type: SignalHandlingType,
    // Signals which the process will ignore when its in the signal handler.
    pub ignored_sigs_in_sig_handler: Signals,
}

#[derive(Default, Clone, Debug)]
#[repr(C)]
pub enum SignalHandlingType {
    #[default]
    Default,
    Ignore,
    Function(SignalHandlerFn),
}

impl From<u64> for SignalHandlingType {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Ignore,
            _ => Self::Function(unsafe {
                core::mem::transmute::<usize, SignalHandlerFn>(value as usize)
            }),
        }
    }
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

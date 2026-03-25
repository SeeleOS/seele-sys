use bitflags::bitflags;
use num_enum::TryFromPrimitive;
use strum::EnumIter;

#[derive(Clone, Copy, TryFromPrimitive, Debug, EnumIter)]
#[repr(u64)]
pub enum Signal {
    Terminate = 0,
    Kill,
    Interrupt,
}

pub const SIGNAL_AMOUNT: usize = 3;

pub type SignalHandlerFn = extern "C" fn(i32);

/// The action that a process will take when it got a signal.
#[derive(Default, Clone, Debug)]
#[repr(C)]
pub struct SignalAction {
    pub handling_type: SignalHandlingType,
    // Signals which the process will ignore when its in the signal handler.
    pub ignored_signals: Signals,
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
    }
}

impl From<Signal> for Signals {
    fn from(value: Signal) -> Self {
        match value {
            Signal::Terminate => Self::TERMINATE,
            Signal::Kill => Self::KILL,
            Signal::Interrupt => Self::INTERRUPT,
        }
    }
}

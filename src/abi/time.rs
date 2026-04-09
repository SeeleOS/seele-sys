use num_enum::TryFromPrimitive;

use crate::signal::Signal;

#[derive(TryFromPrimitive, Debug, Clone, Copy, Default, Eq, PartialEq)]
#[repr(u64)]
pub enum TimeType {
    Realtime,
    #[default]
    SinceBoot,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum TimerNotifyType {
    #[default]
    None = 0,
    Signal = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TimerNotifyStruct {
    pub notify_type: TimerNotifyType,
    pub signal: Signal,
}

impl Default for TimerNotifyStruct {
    fn default() -> Self {
        Self {
            notify_type: TimerNotifyType::None,
            signal: Signal::Alarm,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(u64)]
pub enum TimerStateType {
    #[default]
    Disabled = 0,
    OneShot,
    Periodic,
}

#[derive(Default, Clone, Copy, Debug)]
#[repr(C)]
pub struct TimerStateStruct {
    pub state_type: TimerStateType,
    pub deadline: u64,
    pub interval: u64,
}

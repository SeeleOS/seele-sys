#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum TimeType {
    Realtime,
    #[default]
    SinceBoot,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum TimerNotifyMethod {
    #[default]
    None,
}

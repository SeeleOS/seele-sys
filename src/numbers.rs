use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(usize)]
pub enum SyscallNumber {
    Print = 1000,
    SetFs,
    SetGs,
    GetFs,
    AllocateMem,
    GetProcessID,
    GetThreadID,
    FutexWait,
    FutexWake,
    Exit,
    ReadObject,
    WriteObject,
    ConfigurateObject,
    ChangeDirectory,
    GetCurrentDirectory,
    FileInfo,
    Fork,
    Execve,
    OpenFile,
    RemoveObject,
    WaitForProcessExit,
    GetDirectoryContents,
    GetProcessParentID,
    ControlObject,
    CreatePoller,
    PollerAdd,
    PollerRemove,
    PollerWait,

    CloneObject,
    CloneObjectTo,
    MapFile,
    RegisterSignalAction,
    SendSignal,

    UpdateMemPerms,

    DeallocateMem,
}

impl SyscallNumber {
    pub fn from_number(number: usize) -> Option<Self> {
        Self::try_from(number).ok()
    }
}

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
    _Unused2,
    RegisterSignalAction,
    SendSignal,

    UpdateMemPerms,

    DeallocateMem,

    _Unused,

    BlockSignals,
    UnblockSignals,
    SetBlockedSignals,

    SigHandlerReturn,

    GetSystemInfo,

    GetCurrentTime,
    TimeSinceBoot,

    OpenDevice,

    MmapObject,
    Socket,
    SocketBind,
    SocketListen,
    SocketConnect,
    SocketAccept,
    SocketGetSockOpt,
    DeleteFile,
    LinkFile,
    Sleep,
    CreateDirectory,

    SeekObject,

    ThreadClone,

    ReadLink,

    SendSignalGroup,

    SetProcessGroupID,
    GetProcessGroupID,
    SocketGetSockName,
    SocketGetPeerName,
    SocketRecvMsg,
    SocketShutdown,

    SendSignalToAll,

    CreateTimer,
    DeleteTimer,
    SetTimerState,
    GetTimerState,
    GetTimerOverrun,

    CreatePty,
}

impl SyscallNumber {
    pub fn from_number(number: usize) -> Option<Self> {
        Self::try_from(number).ok()
    }
}

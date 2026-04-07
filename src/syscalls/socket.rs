use crate::{SyscallResult, syscall};

pub fn socket(domain: u64, kind: u64, protocol: u64) -> SyscallResult {
    syscall!(Socket, domain, kind, protocol)
}

pub fn bind(socket: u64, path: *const i8) -> SyscallResult {
    syscall!(SocketBind, socket, path as u64)
}

pub fn listen(socket: u64, backlog: u64) -> SyscallResult {
    syscall!(SocketListen, socket, backlog)
}

pub fn connect(socket: u64, path: *const i8) -> SyscallResult {
    syscall!(SocketConnect, socket, path as u64)
}

pub fn accept(socket: u64) -> SyscallResult {
    syscall!(SocketAccept, socket)
}

pub fn getsockopt(
    socket: u64,
    level: u64,
    option_name: u64,
    option_value: *mut u8,
    option_len: *mut u32,
) -> SyscallResult {
    syscall!(
        SocketGetSockOpt,
        socket,
        level,
        option_name,
        option_value as u64,
        option_len as u64
    )
}

pub fn getsockname(socket: u64, address: *mut u8, address_len: *mut u32) -> SyscallResult {
    syscall!(
        SocketGetSockName,
        socket,
        address as u64,
        address_len as u64
    )
}

pub fn getpeername(socket: u64, address: *mut u8, address_len: *mut u32) -> SyscallResult {
    syscall!(
        SocketGetPeerName,
        socket,
        address as u64,
        address_len as u64
    )
}

pub fn recvmsg(socket: u64, msg: *mut u8, flags: u64) -> SyscallResult {
    syscall!(SocketRecvMsg, socket, msg as u64, flags)
}

pub fn shutdown(socket: u64, how: u64) -> SyscallResult {
    syscall!(SocketShutdown, socket, how)
}

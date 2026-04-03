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

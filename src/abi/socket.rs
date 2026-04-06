pub const AF_UNIX: u64 = 1;

pub const SOL_SOCKET: u64 = 1;

pub const SOCK_STREAM: u64 = 1;
pub const SOCK_DGRAM: u64 = 2;
pub const SOCK_NONBLOCK: u64 = 0o4_000;
pub const SOCK_CLOEXEC: u64 = 0o2_000_000;

pub const SO_REUSEADDR: u64 = 2;
pub const SO_TYPE: u64 = 3;
pub const SO_ERROR: u64 = 4;
pub const SO_SNDBUF: u64 = 7;
pub const SO_RCVBUF: u64 = 8;
pub const SO_PASSCRED: u64 = 16;
pub const SO_PEERCRED: u64 = 17;
pub const SO_ACCEPTCONN: u64 = 30;
pub const SO_PROTOCOL: u64 = 38;
pub const SO_DOMAIN: u64 = 39;

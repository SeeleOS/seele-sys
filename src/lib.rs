#![no_std]

pub mod errors;
pub mod misc;
pub mod numbers;
pub mod permission;
pub mod signal;
pub mod syscalls;
pub mod utils;

pub use utils::SyscallResult;

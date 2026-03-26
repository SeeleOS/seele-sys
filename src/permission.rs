bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Permissions: u64 {
        const READABLE = 1 << 0;
        const WRITABLE = 1 << 1;
        const EXECUTABLE = 1 << 2;
    }
}

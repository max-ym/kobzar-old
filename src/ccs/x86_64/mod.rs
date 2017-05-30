/// Module to setup x86_64 architecture CCS.
mod setup;
pub use self::setup::setup;
pub use super::Service;

/// All information needed to execute the service. Memory pages, register file
/// stack etc.
pub struct ServiceData {

    /// General information about service.
    base    : Service,
}

/// General purpose register file.
struct GpRegisterFile {
    rax : u64,
    rbx : u64,
    rcx : u64,
    rdx : u64,
    rsi : u64,
    rdi : u64,
    r8  : u64,
    r9  : u64,
    r10 : u64,
    r11 : u64,
    r12 : u64,
    r13 : u64,
    r14 : u64,
    r15 : u64,
}

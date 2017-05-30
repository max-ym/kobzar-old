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

struct Xmm(u64, u64);

/// XMM register file.
struct XmmRegisterFile {
    xmm0    : Xmm,
    xmm1    : Xmm,
    xmm2    : Xmm,
    xmm3    : Xmm,
    xmm4    : Xmm,
    xmm5    : Xmm,
    xmm6    : Xmm,
    xmm7    : Xmm,
    xmm8    : Xmm,
    xmm9    : Xmm,
    xmm10   : Xmm,
    xmm11   : Xmm,
    xmm12   : Xmm,
    xmm13   : Xmm,
    xmm14   : Xmm,
    xmm15   : Xmm,
}

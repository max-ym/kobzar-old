/// General purpose register file.
#[derive(Default, Clone, Copy)]
pub struct GpRegisterFile {
    pub rax : u64,
    pub rbx : u64,
    pub rcx : u64,
    pub rdx : u64,
    pub rsi : u64,
    pub rdi : u64,
    pub r8  : u64,
    pub r9  : u64,
    pub r10 : u64,
    pub r11 : u64,
    pub r12 : u64,
    pub r13 : u64,
    pub r14 : u64,
    pub r15 : u64,

    pub rsp : u64,
    pub rip : u64,
    pub rf  : u64, // RFLAGS
}

/// XMM register.
#[derive(Default, Clone, Copy)]
pub struct Xmm(u64, u64);

/// XMM register file.
#[derive(Default, Clone, Copy)]
pub struct XmmRegisterFile {
    pub xmm0    : Xmm,
    pub xmm1    : Xmm,
    pub xmm2    : Xmm,
    pub xmm3    : Xmm,
    pub xmm4    : Xmm,
    pub xmm5    : Xmm,
    pub xmm6    : Xmm,
    pub xmm7    : Xmm,
    pub xmm8    : Xmm,
    pub xmm9    : Xmm,
    pub xmm10   : Xmm,
    pub xmm11   : Xmm,
    pub xmm12   : Xmm,
    pub xmm13   : Xmm,
    pub xmm14   : Xmm,
    pub xmm15   : Xmm,
}

/// YMM register.
#[derive(Default, Clone, Copy)]
pub struct Ymm(u64, u64, u64, u64);

/// YMM register file.
#[derive(Default, Clone, Copy)]
pub struct YmmRegisterFile {
    pub ymm0    : Ymm,
    pub ymm1    : Ymm,
    pub ymm2    : Ymm,
    pub ymm3    : Ymm,
    pub ymm4    : Ymm,
    pub ymm5    : Ymm,
    pub ymm6    : Ymm,
    pub ymm7    : Ymm,
    pub ymm8    : Ymm,
    pub ymm9    : Ymm,
    pub ymm10   : Ymm,
    pub ymm11   : Ymm,
    pub ymm12   : Ymm,
    pub ymm13   : Ymm,
    pub ymm14   : Ymm,
    pub ymm15   : Ymm,
}

/// Floating-point register.
#[derive(Default, Clone, Copy)]
#[repr(packed)]
pub struct Fpr(u16, u64);

/// Floating-point register register file.
#[derive(Default, Clone, Copy)]
pub struct FprRegisterFile {
    pub fpr0    : Fpr,
    pub fpr1    : Fpr,
    pub fpr2    : Fpr,
    pub fpr3    : Fpr,
    pub fpr4    : Fpr,
    pub fpr5    : Fpr,
    pub fpr6    : Fpr,
    pub fpr7    : Fpr,
}

/// MMX register file.
#[derive(Default, Clone, Copy)]
pub struct MmxRegisterFile {
    pub mmx0    : u64,
    pub mmx1    : u64,
    pub mmx2    : u64,
    pub mmx3    : u64,
    pub mmx4    : u64,
    pub mmx5    : u64,
    pub mmx6    : u64,
    pub mmx7    : u64,
}
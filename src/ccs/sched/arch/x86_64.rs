use super::*;
use arch;

/// Processor Data Table. Each entry corresponds to local APIC ID which
/// identifies processor ID.
#[repr(packed)]
pub struct Pdt {
    arr     : [*mut ProcessorData; 256],
}

/// Processor-specific data. Struct stores processors environment
/// variables and settings. This struct is accessed by assembly code so
/// be careful with making changes to it. See 'main.fasm'.
#[repr(packed)]
pub struct ProcessorData {

    /// Address for register file backup on context switch.
    regs    : u64,

    /// Flags of kernel processor settings.
    flags   : u32,
}

impl ProcessorData {

    /// Flag to save SSE registers.
    const F_SAVE_SSE            : u32 = (1 << 0x00);

    /// Flag to save MMX registers. If F_SAVE_FP is set it overrides
    /// this flag. MMX registers are sharing same physical registers
    /// as FP registers. But MMX are 64-bit instead of 80-bit FP.
    /// FP will save all MMX register values + another 16-bits
    /// that are not covered by MMX.
    const F_SAVE_MMX            : u32 = (1 << 0x01);

    /// Flag to save FP registers.
    const F_SAVE_FP             : u32 = (1 << 0x02);

    /// Flag to save general purpose registers. May be off when
    /// core is halted and does not run particular process.
    const F_SAVE_GP             : u32 = (1 << 0x03);
}

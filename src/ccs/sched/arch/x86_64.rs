use super::*;
use arch;
use arch::xsave;

/// Processor Data Table. Each entry corresponds to local APIC ID which
/// identifies processor ID. This is Kobzar arhitectural struct, not
/// related to Intel arch.
/// It supports currently up to 256 processors or cores.
///
/// This struct is used by assebly routine isr_sched_process_change.
/// When making changes don't forget to reflect them to FASM code.
#[repr(packed)]
pub struct Pdt {
    arr     : [*mut ProcessorData; 256],
}

/// Processor-specific data. Struct stores processors environment
/// variables and settings. This struct is accessed by assembly code so
/// be careful with making changes to it. See 'main.fasm', routine
/// isr_sched_process_change.
#[repr(packed)]
pub struct ProcessorData {

    /// Address for XSAVE area. State of current process on context switch
    /// must be saved by this address.
    xsave   : u64,

    /// XSAVE mask to use for backup operations.
    xmask   : xsave::Mask,

    /// General purpose register file and state file address.
    /// State of current process on context switch
    /// must be saved by this address.
    gpregs  : *mut (arch::regf::GeneralPurpose, arch::regf::State),

    /// Flags of kernel processor settings.
    flags   : PdFlags,
}

/// Processor Data flags. Flags of kernel processor settings.
#[repr(packed)]
#[derive(Clone, Copy, Default)]
struct PdFlags {
    val     : u32
}

#[repr(packed)]
pub struct IsrStack {
    ss      : u64,
    rsp     : u64,
    rflags  : u64,
    cs      : u64,
    rip     : u64
}

impl PdFlags {

    /// Save general purpose registers on context switch. May be off
    /// if there is no need to save this content like when processor
    /// is halted.
    const SAVE_GP       : u32 = 1 << 0x0;

    /// Whether GP registers need to be saved.
    pub fn need_gp_save(&self) -> bool {
        self.val & Self::SAVE_GP != 0
    }

    /// Set flag to save GP registers.
    pub fn set_gp_save(&mut self) {
        self.val |= Self::SAVE_GP;
    }

    /// Unset flag so that GP registers wont be saved.
    pub fn unset_gp_save(&mut self) {
        self.val &= !Self::SAVE_GP;
    }
}

impl ProcessorData {

}

/// Rust part of IST that handles process change signal
/// from the scheduler. Assembler routine saves GP registers
/// (if corresponding flag is set) before calling
/// this function so that GP regisers
/// could be safely used here.
#[no_mangle]
pub extern "C" fn rust_isr_sched_process_change(
        stk: *mut IsrStack, data: *mut ProcessorData) {
    // GP registers are already saved by assembler routine.

    let data = unsafe { &mut *data };
    let stk  = unsafe { &mut *stk  };

    // Save current process state.
    xsave::xsaves(data.xsave, data.xmask);

    // TODO load next process data.

    unimplemented!()
}

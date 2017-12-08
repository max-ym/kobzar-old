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

    /// Address for XSAVE area.
    xsave   : u64,

    /// XSAVE mask to use for backup operations.
    xmask   : xsave::Mask,

    /// General purpose register file address.
    gpregs  : *mut arch::regf::GeneralPurpose,

    /// Flags of kernel processor settings.
    flags   : PdFlags,
}

/// Processor Data flags. Flags of kernel processor settings.
#[repr(packed)]
#[derive(Clone, Copy, Default)]
struct PdFlags {
    val     : u32
}

impl ProcessorData {

}

/// Rust part of IST that handles process change signal
/// from the scheduler. Assembler routine saves GP registers
/// (if corresponding flag is set) before calling
/// this function so that GP regisers
/// could be safely used here.
#[no_mangle]
pub extern fn rust_isr_sched_process_change(data: *mut ProcessorData) {
    // GP registers are already saved by assembler routine.

    use self::xsave::Mask;
    let pd = unsafe { &mut *data };

    // TODO impl more flags.

    unimplemented!()
}

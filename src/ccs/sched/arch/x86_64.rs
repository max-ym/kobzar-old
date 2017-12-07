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

    /// General purpose register file address.
    gpregs  : *mut arch::regf::GeneralPurpose,

    /// Flags of kernel processor settings.
    flags   : u32,
}

macro_rules! pdsaves_impl {
    ($cons:ident, $check:ident, $set:ident, $unset:ident, $docs:expr) => (
        #[doc=$docs]
        pub fn $check(&self) -> bool {
            self.flags & Self::$cons != 0
        }

        /// Set corresponding flag.
        pub fn $set(&mut self) {
            self.flags |= Self::$cons;
        }

        /// Clear corresponding flag.
        pub fn $unset(&mut self) {
            self.flags &= !Self::$cons;
        }
    );

    ($cons:ident, $check:ident, $set:ident, $unset:ident) => (
        pdsaves_impl!($cons, $check, $set, $unset)
    );
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

    /// Flag to save FP registers. If F_SAVE_SSE is set, this
    /// flag state is ignored and treated as set.
    const F_SAVE_FP             : u32 = (1 << 0x02);

    /// Flag to save general purpose registers. May be off when
    /// core is halted and does not run particular process.
    const F_SAVE_GP             : u32 = (1 << 0x03);

    pdsaves_impl!(F_SAVE_SSE, is_sse_saved,
        do_save_sse, dont_save_sse,
        "Check save SSE flag."
    );
    pdsaves_impl!(F_SAVE_MMX, is_mmx_saved,
        do_save_mmx, dont_save_mmx,
        "Check save MMX flag."
    );
    pdsaves_impl!(F_SAVE_FP, is_fp_saved,
        do_save_fp, dont_save_fp,
        "Check save FP flag."
    );
    pdsaves_impl!(F_SAVE_GP, is_gp_saved,
        do_save_gp, dont_save_gp,
        "Check save GP flag."
    );
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

    let mut mask = Mask::default();

    if pd.is_sse_saved() {
        mask.enable_sse();
    }

    // TODO impl more flags.

    unimplemented!()
}

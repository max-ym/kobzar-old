/// Local APIC related functions and data.
pub struct LocalApic {
    /// Local APIC MSR access.
    msr     : ::asm::msr::ApicBase,
}

impl LocalApic {

    /// Create new structure to hold information about Local APIC state.
    /// Note that this will read MSR that may be unimplemented which will
    /// cause General Protection fault.
    ///
    /// You can use 'new' that will check if APIC is actually available
    /// before any read to MSR.
    pub unsafe fn unsafe_new() -> LocalApic {
        let msr = ::asm::msr::ApicBase::read();

        LocalApic { msr:msr }
    }

    pub fn new() -> Option<LocalApic> {
        if !Self::is_present() {
            None
        } else {
            unsafe { Some(Self::unsafe_new()) }
        }
    }

    pub fn is_present() -> bool {
        ::asm::cpuid::Features::get().check_local_apic_presence()
    }
}

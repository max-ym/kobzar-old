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

        LocalApic {
            msr:msr,
            was_disabled: false
        }
    }

    /// Check if Local APIC exists. If this device is there, create
    /// a structure to represent it.
    pub fn new() -> Option<LocalApic> {
        if !Self::is_present() {
            None
        } else {
            unsafe { Some(Self::unsafe_new()) }
        }
    }

    /// Check if Local APIC is present by calling CPUID instruction.
    pub fn is_present() -> bool {
        ::asm::cpuid::Features::get().check_local_apic_presence()
    }

    pub fn is_global_enabled(&self) -> bool {
        self.msr.apic_global_enabled()
    }

    /// Enable Local APIC.
    pub fn global_enable(&mut self) {
        self.msr.apic_global_enable();
        self.msr.write();
    }

    /// Disable Local APIC if it is enabled.
    pub fn global_disable(&mut self) {
        if self.is_global_enabled() {
            self.msr.apic_global_disable();
            unsafe { self.msr.write(); }
        }
    }
}

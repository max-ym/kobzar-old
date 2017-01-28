/// Local APIC related functions and data.
pub struct LocalApic {

    /// Local APIC MSR access.
    msr     : ::asm::msr::ApicBase,

    /// Once APIC was disabled, due to technical reasons, it may not
    /// be enabled again to perform properly on some architectures.
    ///
    /// TODO: currently there is no architectural check that verifies
    /// if it could be re-enabled again. It may be programed later when needed.
    was_disabled    : bool,
}

// TODO: only once this structure may be created. Make it a singleton.

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

    pub fn was_disabled(&self) -> bool {
        self.was_disabled
    }

    /// Enable APIC. Do not check if APIC can be safely enabled.
    pub unsafe fn unsafe_enable(&mut self) {
        self.msr.apic_global_enable();
        self.msr.write();
    }

    /// Try to enable Local APIC. Check if it is safe to do so.
    pub fn enable(&mut self) -> Result<(),()> {
        if self.was_disabled {
            return Err(());
        }

        // Seems it is safe to enable it now.
        unsafe { self.unsafe_enable() }
        Ok(())
    }

    pub fn disable(&mut self) {
        if self.is_global_enabled() {
            self.was_disabled();
            self.msr.apic_global_disable();
            unsafe { self.msr.write(); }
        }
    }
}

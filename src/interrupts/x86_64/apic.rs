use ::asm::msr::ApicBase;
use ::asm::cpuid;
use ::asm::lowmem::APIC_BASE_ADDRESS;

/// Shows if Local APIC is available on current machine.
/// The initial value is not correct before module initialization.
static mut APIC_PRESENT: bool = false;

/// Initialize the APIC module.
pub fn init() {
    // Check if Local APIC is availabale.
    if cpuid::Features::get().local_apic_is_present() {
        unsafe { APIC_PRESENT = true; }
    } else {
        unsafe { APIC_PRESENT = false; }

        // Nothing to initalize as there is no Local APIC.
        return;
    }

    // Get APIC Base MSR. It is safe to do so as we checked that Local APIC
    // is really present.
    let mut msr = unsafe { ApicBase::read() };

    // Move the APIC registers to Low Memory.
    msr.set_apic_base(APIC_BASE_ADDRESS);

    // Save the MSR changes.
    unsafe { msr.write() };
}

/// Get Local APIC present bit state. This bit is set to correct
/// value after module initialization.
pub fn local_apic_present() -> bool {
    unsafe { APIC_PRESENT }
}

/// Local APIC registers representation.
pub struct LocalApic {
    // The registers are accessed by other functions and are not listed.
}

impl LocalApic {

    /// Get Local APIC access.
    pub fn get() -> Option<&'static LocalApic> {
        if local_apic_present() {
            let ptr = APIC_BASE_ADDRESS as *mut LocalApic;
            unsafe { Some(&(*ptr)) }
        } else {
            None
        }
    }
}

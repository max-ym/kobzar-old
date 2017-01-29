use ::asm::cpuid;
use ::asm::msr;

// TODO multi-threading safety.

static mut MSR_APIC_BASE        : Option<msr::ApicBase>         = None;

// Do not run this unless you ensured that this MSR is present.
fn load_apic_base() {
    unsafe { MSR_APIC_BASE = Some(msr::ApicBase::read()) }
}

/// Early setup of feature lists.
pub mod setup {

    use super::*;
    use ::early::{Logger, LoggerTrait};

    /// Load list with features that are received from CPUID.
    pub fn load_feature_lists(logger: &mut Logger) {
        logger.println("Buffering basic MSR feature list:");

        // Check if APIC_BASE MSR is present.
        if cpuid::Features::get().local_apic_is_present() {
            logger.println(" * APIC Base");
            load_apic_base();
        } else {
            logger.println(" - Didn't load APIC Base (not present)");
        }
    }
}

use ::asm::cpuid;
use ::asm::msr;

// TODO multi-threading safety.

static mut CPUID_VENDOR_STRING  : Option<cpuid::VendorString>   = None;
static mut CPUID_FEATURES       : Option<cpuid::Features>       = None;
static mut CPUID_TLB            : Option<cpuid::Tlb>            = None;
static mut CPUID_SERIAL         : Option<cpuid::Serial>         = None;

static mut MSR_APIC_BASE        : Option<msr::ApicBase>         = None;

fn get_apic_base() -> &'static msr::ApicBase {
    unsafe { &MSR_APIC_BASE.as_ref().unwrap() }
}

fn load_vendor_string() {
    unsafe { CPUID_VENDOR_STRING = Some(cpuid::VendorString::get()) }
}

fn load_features_cpuid() {
    unsafe { CPUID_FEATURES = Some(cpuid::Features::get()) }
}

fn load_tlb() {
    unsafe { CPUID_TLB = Some(cpuid::Tlb::get()) }
}

fn load_serial() {
    unsafe { CPUID_SERIAL = Some(cpuid::Serial::get()) }
}

// Do not run this unless you ensured that this MSR is present.
fn load_apic_base() {
    unsafe { MSR_APIC_BASE = Some(msr::ApicBase::read()) }
}

/// Get CPUID Features.
///
/// # Safety
/// The function will fail only if this function gets runned before feature
/// list is loaded. Do not run the function before its corresponding
/// static variable is set to some value.
pub fn features() -> cpuid::Features {
    unsafe { CPUID_FEATURES.unwrap() }
}

/// Early setup of feature lists.
pub mod setup {

    use super::*;
    use ::early::{Logger, LoggerTrait};

    /// Load list with features that are received from CPUID.
    pub fn load_feature_lists(logger: &mut Logger) {
        logger.println("Buffering basic CPUID feature list:");

        logger.println(" * Vendor String");
        load_vendor_string();

        logger.println(" * Features");
        load_features_cpuid();

        logger.println(" * TLB");
        load_tlb();

        logger.println(" * Serial");
        load_serial();

        // Start loading MSRs:
        logger.println("\nBuffering basic MSR feature list:");

        // Check if APIC_BASE MSR is present.
        if features().local_apic_is_present() {
            logger.println(" * APIC Base");
            load_apic_base();
        } else {
            logger.println(" - Didn't load APIC Base (not present)");
        }
    }
}

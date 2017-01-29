use ::asm::cpuid;
use ::asm::msr;

// TODO multi-threading safety.

static mut CPUID_VENDOR_STRING  : Option<cpuid::VendorString>   = None;
static mut CPUID_FEATURES       : Option<cpuid::Features>       = None;
static mut CPUID_TLB            : Option<cpuid::Tlb>            = None;
static mut CPUID_SERIAL         : Option<cpuid::Serial>         = None;

static mut MSR_APIC_BASE        : Option<msr::ApicBase>         = None;

#[allow(non_upper_case_globals)]
pub static mut apic_base        : fn() -> &'static msr::ApicBase
                                = init_apic_base;

fn init_apic_base() -> &'static msr::ApicBase {
    // TODO properly deal with unsafety!
    unsafe {
    MSR_APIC_BASE   = Some(msr::ApicBase::read());
    apic_base       = get_apic_base;
    apic_base()
    }
}

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

/// Early setup of feature lists.
pub mod setup {

    use super::*;
    use ::early::{Logger, LoggerTrait};

    /// Load list with features that are received from CPUID.
    pub fn load_cpuids(logger: &mut Logger) {
        logger.println("Buffering basic CPUID feature list:");

        logger.println(" * Vendor String");
        load_vendor_string();

        logger.println(" * Features");
        load_features_cpuid();

        logger.println(" * TLB");
        load_tlb();

        logger.println(" * Serial");
        load_serial();
    }
}

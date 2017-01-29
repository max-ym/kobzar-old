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

use ::asm::cpuid;
use ::asm::msr;

static CPUID_VENDOR_STRING  : Option<cpuid::VendorString>   = None;
static CPUID_FEATURES       : Option<cpuid::Features>       = None;
static CPUID_TLB            : Option<cpuid::Tlb>            = None;
static CPUID_SERIAL         : Option<cpuid::Serial>         = None;

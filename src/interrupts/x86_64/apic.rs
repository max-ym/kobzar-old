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

/// List of all local APIC registers and their addresses.
#[repr(u64)]
enum LocalApicReg {
    Id                      = 0x020, // RW (Nehalem RO)
    Version                 = 0x030, // RO

    TaskPriority            = 0x080, // RW
    ArbitrationPriority     = 0x090, // RO, 1
    ProcessorPriority       = 0x0A0, // RO
    Eoi                     = 0x0B0, // WO
    RemoteRead              = 0x0C0, // RO, 1
    LogicalDestination      = 0x0D0, // RW
    DestinationFormat       = 0x0E0, // RW
    SpuriousInterruptVector = 0x0F0, // RW
    Isr0                    = 0x100, // RO
    Isr1                    = 0x110, // RO
    Isr2                    = 0x120, // RO
    Isr3                    = 0x130, // RO
    Isr4                    = 0x140, // RO
    Isr5                    = 0x150, // RO
    Isr6                    = 0x160, // RO
    Isr7                    = 0x170, // RO
    Tmr0                    = 0x180, // RO
    Tmr1                    = 0x190, // RO
    Tmr2                    = 0x1A0, // RO
    Tmr3                    = 0x1B0, // RO
    Tmr4                    = 0x1C0, // RO
    Tmr5                    = 0x1D0, // RO
    Tmr6                    = 0x1E0, // RO
    Tmr7                    = 0x1F0, // RO
    Irr0                    = 0x200, // RO
    Irr1                    = 0x210, // RO
    Irr2                    = 0x220, // RO
    Irr3                    = 0x230, // RO
    Irr4                    = 0x240, // RO
    Irr5                    = 0x250, // RO
    Irr6                    = 0x260, // RO
    Irr7                    = 0x270, // RO
    ErrorStatus             = 0x280, // RO

    LvtCmci                 = 0x2F0, // RW
    InterruptCommand0       = 0x300, // RW
    InterruptCommand1       = 0x310, // RW
    LvtTimer                = 0x320, // RW
    LvtThermalSensor        = 0x330, // RW, 2
    LvtPerformanceCounters  = 0x340, // RW, 3
    LvtLint0                = 0x350, // RW
    LvtLint1                = 0x360, // RW
    LvtError                = 0x370, // RW
    InitialCount            = 0x380, // RW
    CurrentCount            = 0x390, // RO

    DivideConfiguration     = 0x3E0, // RW
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

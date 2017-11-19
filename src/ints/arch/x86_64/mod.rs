use mem::map::IDT as IDT_ADDR;
use mem::map::APIC_BASE_ADDRESS;
use arch::mem;
use arch::idt::*;
use arch::apic;
use arch::apic::LocalApic;
use arch::pic::Pic;

static mut LAPIC_ADDR: ::mem::Address = ::mem::Address::null();

/// Interrupt vectors of the kernel core. Vectors from 0 to 31 are defined
/// by architecture specs and are not listed here.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum KernelVector {

    /// PIT timer interrupt.
    Pit         = 32,
    Keyboard    = 33,
    Cascade     = 34, // Never raised.
    Com2        = 35,
    Com1        = 36,
    Lpt2        = 37,
    Floppy      = 38,
    Spurious    = 39, // Also Lpt1.
    CmosClock   = 40,
    Scsi0       = 41,
    Scsi1       = 42,
    Scsi2       = 43,
    Ps2Mouse    = 44,
    Fpu         = 45,
    AtaPrimary  = 46,
    AtaSecond   = 47,


    /// APIC spurious interrupt.
    /// Must be 0xFF (255).
    ApicSpurious = 255,
}

/// IDT reference.
fn idt() -> &'static Idt {
    unsafe { &*(IDT_ADDR as *const Idt) }
}

/// IDT mutable reference.
fn idt_mut() -> &'static mut Idt {
    unsafe { &mut *(IDT_ADDR as *const Idt as *mut Idt) }
}

/// Local APIC reference.
fn apic() -> &'static LocalApic {
    unsafe { LAPIC_ADDR.as_ref() }
}

/// Local APIC mutbale reference.
fn apic_mut() -> &'static mut LocalApic {
    unsafe { LAPIC_ADDR.as_ref_mut() }
}

/// Initialize IDT and APIC.
fn init() {
    // Zero all bytes of IDT table. This makes all entries treated as
    // unexisting.
    mem::stosq(IDT_ADDR as _, 0, 4096 / 8);

    // Allocate APIC interface.
    unsafe {
        use mem::{Allocator, AllocatorAlign, main_alloc_mut};

        main_alloc_mut().align(8);
        LAPIC_ADDR = main_alloc_mut().alloc_for::<LocalApic>();
    }

    // Try to initialize APIC interface.
    let option = LocalApic::new();
    if option.is_none() {
        panic!("APIC is not supported but needed by Kobzar implementation");
    }
    *apic_mut() = option.unwrap();

    // Create PIC interface.
    let pic = Pic::new();

    // Remap PIC. PIC will be disabled but spurious interrupts still may be
    // generated. Remap needed for spurious interrupts to be delivered in a
    // place that does not overlap with software interrupts.
    pic.remap(0x20, 0x28);

    // Disable PIC. It is neccessary to properly use APIC.
    pic.disable();

    unsafe {
        // Remap APIC to defined base address.
        apic_mut().set_base_addr(APIC_BASE_ADDRESS as _);
    }

    unimplemented!();

    // Copy spurious interrupt register.
    let mut spurious = apic().spurious_interrupt().clone();
    // Set vector for spurious interrupts.
    spurious.set_vector(KernelVector::ApicSpurious as _);
    // Save changes.
    *apic_mut().spurious_interrupt_mut() = spurious;
}

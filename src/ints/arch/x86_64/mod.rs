use mem::map::IDT as IDT_ADDR;
use arch::mem;
use arch::idt::*;
use arch::apic;
use arch::pic::Pic;

static mut LAPIC_ADDR: ::mem::Address = ::mem::Address::null();

/// Interrupt vectors of the kernel core. Vectors from 0 to 31 are defined
/// by architecture specs and are not listed here.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum KernelVector {

    /// PIT timer interrupt.
    Pit = 32,

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
fn apic() -> &'static apic::LocalApic {
    unsafe { LAPIC_ADDR.as_ref() }
}

/// Local APIC mutbale reference.
fn apic_mut() -> &'static mut apic::LocalApic {
    unsafe { LAPIC_ADDR.as_ref_mut() }
}

/// Initialize IDT and APIC.
fn init() {
    let idt = idt_mut();

    // Zero all bytes of IDT table. This makes all entries treated as
    // unexisting.
    mem::stosq(IDT_ADDR as _, 0, 4096 / 8);

    // Disable PIC. It is neccessary to properly use APIC.
    Pic::new().disable();

    // Allocate APIC interface.
    unsafe {
        use mem::{Allocator, AllocatorAlign, main_alloc_mut};

        main_alloc_mut().align(8);
        LAPIC_ADDR = main_alloc_mut().alloc_for::<apic::LocalApic>();
    }

    // Try to initialize APIC interface.
    let option = apic::LocalApic::new();
    if option.is_none() {
        panic!("APIC is not supported but needed by Kobzar implementation");
    }
    *apic_mut() = option.unwrap();

    unimplemented!()
}

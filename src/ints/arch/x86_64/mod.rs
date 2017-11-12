use mem::map::IDT as IDT_ADDR;
use arch::mem;
use arch::idt::*;

/// Interrupt vectors of the kernel core. Vectors from 0 to 31 are defined
/// by architecture specs and are not listed here.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum KernelVectors {

    /// PIT timer interrupt.
    Pit = 32,

    /// APIC spurious interrupt.
    ApicSpurious = 255;
}

/// IDT reference.
fn idt() -> &'static Idt {
    unsafe { &*(IDT_ADDR as *const Idt) }
}

/// IDT mutable reference.
fn idt_mut() -> &'static mut Idt {
    unsafe { &mut *(IDT_ADDR as *const Idt as *mut Idt) }
}

/// Initialize IDT.
fn init_idt() {
    let idt = idt_mut();

    // Zero all bytes of IDT table. This makes all entries treated as
    // unexisting.
    mem::stosq(IDT_ADDR as _, 0, 4096 / 8);
}

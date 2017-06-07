use arch::idt::*;

fn idt() -> &'static mut Idt {
    unsafe { &mut *(::mem::map::IDT as *const Idt as *mut Idt) }
}

extern "C" fn int_divide_error() -> ! {
    panic!("Divide error occured");
}

pub fn setup() {
    use arch::tentr::*;
    use arch::RegValue;

    // Clear memory region.
    unsafe {
        ::core::intrinsics::write_bytes(idt() as *const _ as *mut Idt, 0, 1);
    }

    unimplemented!();
}

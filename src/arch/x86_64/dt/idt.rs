use super::*;

/// General interrupt descriptor table gate.
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IdtGate(i64, i64);

// IDT gate is a of descriptor in IDT.
impl Descriptor for IdtGate {
}

/// Interrupt Descriptor Table.
#[repr(packed)]
pub struct Idt {

    /// The array of all 256 gates of the IDT.
    gates:  [IdtGate; 256],
}

/// Interrupt Descriptor Table handle.
pub struct IdtHandle {
    limit   : u16,
    idt     : *mut Idt,
}

impl Handle for IdtHandle {

    type DescriptorType = IdtGate;

    unsafe fn descriptor_ref<'a, 'b>(&'a self, index: u16)
            -> &'b Self::DescriptorType {
        &*self.gates().offset(index as isize)
    }

    unsafe fn descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> &'b mut Self::DescriptorType {
        &mut *self.gates().offset(index as isize)
    }

    fn limit(&self) -> u16 {
        self.limit
    }

}

impl IdtHandle {

    fn gates(&self) -> *mut IdtGate {
        unsafe { &(*self.idt).gates as *const _ as *mut _ }
    }
}

/// IDTR value.
#[repr(packed)]
pub struct IdtrValue {
    limit   : u16,
    addr    : u64,
}

impl RegValue for IdtrValue {

    type HandleType = IdtHandle;

    unsafe fn write(&self) {
        unimplemented!()
    }

    fn read(&mut self) {
        unimplemented!()
    }

    fn new_from_reg() -> Self {
        unimplemented!()
    }

    fn new(addr: u64, limit: u16) -> Self {
        IdtrValue {
            limit   : limit,
            addr    : addr,
        }
    }

    fn addr(&self) -> u64 {
        self.addr
    }

    fn limit(&self) -> u16 {
        self.limit
    }

    unsafe fn set_addr(&mut self, addr: u64) {
        self.addr = addr;
    }

    unsafe fn set_limit(&mut self, limit: u16) {
        self.limit = limit;
    }

    unsafe fn table(&self) -> Self::HandleType {
        IdtHandle {
            limit   : self.limit,
            idt     : ::core::mem::transmute(self.addr)
        }
    }
}

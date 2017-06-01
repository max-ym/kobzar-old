/// Descriptors module.
pub mod desc;

pub use self::desc::GdtDescriptor;

use super::*;

/// Global Descriptor Table Register value.
#[repr(packed)]
pub struct GdtrValue {
    /// Limit. Number of entries of GDT.
    limit   : u16,

    /// Address of the GDT.
    addr    : u64,
}

impl RegValue for GdtrValue {

    type HandleType = GdtHandle;

    unsafe fn write(&self) {
        unimplemented!();
    }

    fn read(&mut self) {
        unimplemented!();
    }

    fn new_from_reg() -> Self {
        unimplemented!();
    }

    fn new(addr: u64, limit: u16) -> Self {
        GdtrValue {
            addr    : addr,
            limit   : limit,
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
        GdtHandle {
            limit   : self.limit,
            arr     : ::core::mem::transmute(self.addr),
        }
    }
}

/// Global Descriptor Table handle.
pub struct GdtHandle {
    limit   : u16,
    arr    : *mut GdtDescriptor,
}

impl Handle for GdtHandle {

    type DescriptorType = GdtDescriptor;

    unsafe fn descriptor_ref<'a, 'b>(&'a self, index: u16)
            -> &'b Self::DescriptorType {
        &*self.arr.offset(index as isize)
    }

    fn get_descriptor_ref<'a, 'b>(&'a self, index: u16)
            -> Option<&'b Self::DescriptorType> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.descriptor_ref(index) })
        }
    }

    unsafe fn descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> &'b mut Self::DescriptorType {
        &mut *self.arr.offset(index as isize)
    }

    fn get_descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> Option<&'b mut Self::DescriptorType> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.descriptor_mut(index) })
        }
    }

    fn limit(&self) -> u16 {
        self.limit
    }

    fn limit_broken_by(&self, index: u16) -> bool {
        self.limit >= index
    }
}

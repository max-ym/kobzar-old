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

    fn into_table(self) -> Self::HandleType {
        GdtHandle {
            limit   : self.limit,
            arr     : self.addr as *const GdtDescriptor as *mut _,
        }
    }
}

/// Global Descriptor Table handle.
pub struct GdtHandle {
    limit   : u16,
    arr    : *mut GdtDescriptor,
}

impl DtLimit for GdtHandle {
}

impl Table for GdtHandle {

    type EntryType = GdtDescriptor;

    unsafe fn entry_ref<'a, 'b>(&'a self, index: u16)
            -> &'b Self::EntryType {
        &*self.arr.offset(index as isize)
    }

    unsafe fn entry_mut<'a, 'b>(&'a self, index: u16)
            -> &'b mut Self::EntryType {
        &mut *self.arr.offset(index as isize)
    }

    fn limit(&self) -> u16 {
        self.limit
    }

    fn limit_broken_by(&self, index: u16) -> bool {
        <Self as DtLimit>::limit_broken_by(&self, index)
    }
}

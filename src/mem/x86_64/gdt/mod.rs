/// Descriptors module.
pub mod desc;

pub use self::desc::GdtDescriptor;

/// Global Descriptor Table Register value.
#[repr(packed)]
pub struct GdtrValue {
    /// Limit. Number of entries of GDT.
    limit   : u16,

    /// Address of the GDT.
    addr    : u64,
}

impl GdtrValue {

    /// Write current value to the GDTR.
    pub unsafe fn write(&self) {
        unimplemented!();
    }

    /// Read current value from the GDTR.
    pub fn read(&mut self) {
        unimplemented!();
    }

    /// Create GdtrValue struct from current value in GDTR.
    pub fn new_from_gdtr() -> Self {
        unimplemented!();
    }

    pub fn new(addr: u64, limit: u16) -> Self {
        GdtrValue {
            addr    : addr,
            limit   : limit,
        }
    }

    /// Get address of GDT.
    pub fn addr(&self) -> u64 {
        self.addr
    }

    /// Get limit of GDT.
    pub fn limit(&self) -> u16 {
        self.limit
    }

    /// Set address of GDT.
    pub unsafe fn set_addr(&mut self, addr: u64) {
        self.addr = addr;
    }

    /// Set limit of GDT.
    pub unsafe fn set_limit(&mut self, limit: u16) {
        self.limit = limit;
    }

    /// Get Gdt handle from GDTR value.
    pub unsafe fn gdt(&self) -> GdtHandle {
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

impl GdtHandle {

    /// Get descriptor reference by it's index in the descriptor table.
    /// Does not check if descriptor is actually present in the table.
    pub unsafe fn descriptor_ref<'a, 'b>(&'a self, index: u16)
            -> &'b GdtDescriptor {
        &*self.arr.offset(index as isize)
    }

    /// Get descriptor reference by it's index in the descriptor table.
    /// Return None if descriptor is not present.
    pub fn get_descriptor_ref<'a, 'b>(&'a self, index: u16)
            -> Option<&'b GdtDescriptor> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.descriptor_ref(index) })
        }
    }

    /// Get mutable reference to descriptor in GDT by it's index. Does
    /// not check if descriptor is actually present in the table.
    pub unsafe fn descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> &'b mut GdtDescriptor {
        &mut *self.arr.offset(index as isize)
    }

    /// Get mutable reference to descriptor in GDT by it's index.
    /// If descriptor is abscent the None is returned.
    pub fn get_descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> Option<&'b mut GdtDescriptor> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.descriptor_mut(index) })
        }
    }

    /// Get limit of GDT.
    pub fn limit(&self) -> u16 {
        self.limit
    }

    /// Check if given index breaks the limit of GDT. If so, there is no
    /// descriptor with given index in the table.
    pub fn limit_broken_by(&self, index: u16) -> bool {
        self.limit >= index
    }
}

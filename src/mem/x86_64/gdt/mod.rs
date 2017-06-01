/// Descriptors module.
pub mod desc;

pub use self::desc::Descriptor;

/// Global Descriptor Table Register value.
#[repr(packed)]
pub struct GdtrValue {
    /// Address of the GDT.
    addr    : u64,

    /// Limit. Number of entries of GDT.
    limit   : u16,
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
}

/// Global Descriptor Table handle.
pub struct Gdt {
    limit   : u16,
    data    : *mut Descriptor,
}

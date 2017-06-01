/// Descriptors module.
pub mod desc;

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
}

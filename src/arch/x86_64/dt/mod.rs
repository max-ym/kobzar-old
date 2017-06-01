/// Global Descriptor Table related stuff.
pub mod gdt;

/// Interrupt Descriptor Table stuff.
pub mod idt;

/// Descriptor Table Register Value.
pub trait RegValue {

    /// Write current value to appropriate DTR.
    unsafe fn write(&self);


    /// Read current value from appropriate DTR.
    fn read(&mut self);

    /// Create DtrValue struct from current value in DTR.
    fn new_from_reg() -> Self;

    /// Create new value with given base address and limit.
    fn new(addr: u64, limit: u16) -> Self;

    /// Get base address of DT.
    fn addr(&self) -> u64;

    /// Get limit of DT.
    fn limit(&self) -> u16;

    /// Set address of DT.
    unsafe fn set_addr(&mut self, addr: u64);

    /// Set limit of DT.
    unsafe fn set_limit(&mut self, limit: u16);

    /// Get Gdt handle from GDTR value.
    unsafe fn table(&self) -> Handle;
}

/// Descriptor from DescriptorTable.
pub trait Descriptor {
}

/// Descriptor Table handle.
pub trait Handle {

    /// Get descriptor reference by it's index in the descriptor table.
    /// Does not check if descriptor is actually present in the table.
    unsafe fn descriptor_ref<'a, 'b>(&'a self, index: u16) -> &'b Descriptor;

    /// Get descriptor reference by it's index in the descriptor table.
    /// Return None if descriptor is not present.
    fn get_descriptor_ref<'a, 'b>(&'a self, index: u16)
            -> Option<&'b Descriptor>;

    /// Get mutable reference to descriptor in DT by it's index. Does
    /// not check if descriptor is actually present in the table.
    unsafe fn descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> &'b mut Descriptor;

    /// Get mutable reference to descriptor in GDT by it's index.
    /// If descriptor is abscent the None is returned.
    fn get_descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> Option<&'b mut Descriptor>;

    /// Get limit of DT.
    fn limit(&self) -> u16;

    /// Check if given index breaks the limit of DT. If so, there is no
    /// descriptor with given index in the table.
    fn limit_broken_by(&self, index: u16) -> bool {
        self.limit() >= index
    }
}

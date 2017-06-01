/// Global Descriptor Table related stuff.
pub mod gdt;

/// Interrupt Descriptor Table stuff.
pub mod idt;

/// Descriptor Table Register Value.
pub trait DtrValue {

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
    unsafe fn dt(&self) -> DtHandle;
}

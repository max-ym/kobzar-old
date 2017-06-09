use super::tentr::*;

/// Global Descriptor Table related stuff.
pub mod gdt;

/// Interrupt Descriptor Table stuff.
pub mod idt;

/// Descriptor Table Register Value.
pub trait RegValue {

    type HandleType : Table;

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

    /// Consume DTR value and get DT handle.
    fn into_table(self) -> Self::HandleType;
}

/// Descriptor Privilege Level. Used in GDT and IDT.
#[repr(u32)]
pub enum Dpl {
    Dpl0 = 0,
    Dpl1 = 1,
    Dpl2 = 2,
    Dpl3 = 3,
}

/// IA-32e mode descriptor type.
#[repr(u16)]
#[derive(PartialEq)]
pub enum DescriptorType {
    Ldt             = 0b0010,
    TssAvailable    = 0b1001,
    TssBusy         = 0b1011,
    CallGate        = 0b1100,
    InterruptGate   = 0b1110,
    TrapGate        = 0b1111,

    Reserved
}

impl From<u16> for DescriptorType {

    fn from(v: u16) -> Self {
        unsafe { ::core::mem::transmute(v) }
    }
}

/// Descriptor Table entry limit field trait.
///
/// Implements specific limit field functions in descriptors.
/// Designed to be used with 'Table' trait which provides functions with the
/// same name to override them. Implementing this trait lets to use default
/// functions to calculate limit bounds in spite of implementing the same
/// function for each DT entry type individually.
trait DtLimit: Table {

    /// Check if given index breaks the limit of DT. If so, there is no
    /// descriptor with given index in the table.
    fn limit_broken_by(&self, index: u16) -> bool {
        self.limit() < index * Self::EntryType::size() as u16 + 1
    }
}

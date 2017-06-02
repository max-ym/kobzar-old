/// Global Descriptor Table related stuff.
pub mod gdt;

/// Interrupt Descriptor Table stuff.
pub mod idt;

/// Descriptor Table Register Value.
pub trait RegValue {

    type HandleType : Handle;

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
    unsafe fn table(&self) -> Self::HandleType;
}

/// Descriptor from DescriptorTable.
pub trait Descriptor : Sized {

    /// Get size in bytes of a descriptor type.
    fn size() -> usize {
        ::core::mem::size_of::<Self>()
    }
}

/// Specific interpretation of some general descriptor in table. For
/// example, IDT has gates which are descriptors of this table. But
/// those descriptors can be interrupt gates or trap gates. So each
/// of the gates implement this trait to show their specialization and all
/// implement Descriptor trait as both are descriptors of IDT.
pub trait DescriptorVariant<T: Descriptor>: Descriptor {

    /// Try to get reference to a descriptor variant. If it cannot be
    /// interpreted in a requested way, None will be returned.
    fn try_variant_ref(value: &T) -> Option<&Self>;

    /// Try to get mutable reference to a descriptor variant. If it cannot be
    /// interpreted in a requested way, None will be returned.
    fn try_variant_mut(value: &mut T) -> Option<&mut Self>;
}

/// Descriptor Table handle.
pub trait Handle {

    type DescriptorType : Descriptor;

    /// Get descriptor reference by it's index in the descriptor table.
    /// Does not check if descriptor is actually present in the table.
    unsafe fn descriptor_ref<'a, 'b>(&'a self, index: u16)
            -> &'b Self::DescriptorType;

    /// Get descriptor reference by it's index in the descriptor table.
    /// Return None if descriptor is not present.
    fn get_descriptor_ref<'a, 'b>(&'a self, index: u16)
            -> Option<&'b Self::DescriptorType> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.descriptor_ref(index) })
        }
    }

    /// Get mutable reference to descriptor in DT by it's index. Does
    /// not check if descriptor is actually present in the table.
    unsafe fn descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> &'b mut Self::DescriptorType;

    /// Get mutable reference to descriptor in GDT by it's index.
    /// If descriptor is abscent the None is returned.
    fn get_descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> Option<&'b mut Self::DescriptorType> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.descriptor_mut(index) })
        }
    }

    /// Get limit of DT.
    fn limit(&self) -> u16;

    /// Check if given index breaks the limit of DT. If so, there is no
    /// descriptor with given index in the table.
    fn limit_broken_by(&self, index: u16) -> bool {
        self.limit() < index * Self::DescriptorType::size() as u16 + 1
    }
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

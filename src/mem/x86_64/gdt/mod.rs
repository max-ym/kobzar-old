/// Descriptors module.
pub mod desc;

/// Global Descriptor Table Register.
#[repr(packed)]
pub struct Gdtr {
    /// Address of the GDT.
    addr    : u64,

    /// Limit. Number of entries of GDT.
    limit   : u16,
}

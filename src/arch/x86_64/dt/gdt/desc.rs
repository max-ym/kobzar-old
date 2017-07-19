use super::Dpl;

/// Struct to represent general GDT descriptor type.
#[repr(packed)]
pub struct GdtDescriptor {
    data    : [u64; 2]
}

use super::{Entry, EntryVariant};
impl Entry for GdtDescriptor {
}

/// The first descriptor in GDT is null.
#[repr(packed)]
pub struct NullDescriptor {
    null    : [u64; 2]
}

impl Entry for NullDescriptor {
}

impl Default for NullDescriptor {

    fn default() -> Self {
        NullDescriptor {
            null: [0, 0]
        }
    }
}

impl EntryVariant<NullDescriptor> for GdtDescriptor {

    fn try_variant_ref(&self) -> Option<&NullDescriptor> {
        if self.data[0] == 0 && self.data[1] == 0 {
            unsafe { Some(::core::mem::transmute(self)) }
        } else {
            None
        }
    }

    fn try_variant_mut(&mut self) -> Option<&mut NullDescriptor> {
        if self.data[0] == 0 && self.data[1] == 0 {
            unsafe { Some(::core::mem::transmute(self)) }
        } else {
            None
        }
    }
}

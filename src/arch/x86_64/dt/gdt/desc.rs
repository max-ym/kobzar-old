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

#[repr(packed)]
pub struct CallGateDescriptor {
    offset0 : u16,
    segsel  : u16,
    flags   : u16,
    offset1 : u16,
    offset2 : u32,
    resv    : u32,
}

#[repr(packed)]
pub struct TssDescriptor {
    limit   : u16,
    base0   : u16,
    flags0  : u16,
    flags1  : u8 ,
    base1   : u8 ,
    base2   : u32,
    resv    : u32,
}

#[repr(packed)]
pub struct LdtDescriptor {
    limit   : u16,
    base0   : u16,
    flags0  : u16,
    flags1  : u8 ,
    base1   : u8 ,
    base2   : u32,
    resv    : u32,
}

impl Entry for CallGateDescriptor {
}

impl Entry for TssDescriptor {
}

impl Entry for LdtDescriptor {
}

macro_rules! is_cgd_type {
    ($x:ident) => {{
        use super::DescriptorType::CallGate;
        ($x.data[0] & 0x0F000000) >> 8 == CallGate as _
    }};
}

macro_rules! is_tss_type {
    ($x:ident) => {{
        use super::DescriptorType::{TssAvailable, TssBusy};
        let t = ($x.data[0] & 0x0F000000) >> 8;
        t == TssAvailable as _ || t == TssBusy as _
    }};
}

macro_rules! is_ldt_type {
    ($x:ident) => {{
        use super::DescriptorType::{Ldt};
        ($x.data[0] & 0x0F000000) >> 8 == Ldt as _
    }};
}

impl EntryVariant<CallGateDescriptor> for GdtDescriptor {

    fn try_variant_ref(&self) -> Option<&CallGateDescriptor> {
        if is_cgd_type!(self) {
            unsafe { Some(::core::mem::transmute(self)) }
        } else {
            None
        }
    }

    fn try_variant_mut(&mut self) -> Option<&mut CallGateDescriptor> {
        if is_cgd_type!(self) {
            unsafe { Some(::core::mem::transmute(self)) }
        } else {
            None
        }
    }
}

impl EntryVariant<TssDescriptor> for GdtDescriptor {

    fn try_variant_ref(&self) -> Option<&TssDescriptor> {
        if is_tss_type!(self) {
            unsafe { Some(::core::mem::transmute(self)) }
        } else {
            None
        }
    }

    fn try_variant_mut(&mut self) -> Option<&mut TssDescriptor> {
        if is_tss_type!(self) {
            unsafe { Some(::core::mem::transmute(self)) }
        } else {
            None
        }
    }
}

impl EntryVariant<LdtDescriptor> for GdtDescriptor {

    fn try_variant_ref(&self) -> Option<&LdtDescriptor> {
        if is_ldt_type!(self) {
            unsafe { Some(::core::mem::transmute(self)) }
        } else {
            None
        }
    }

    fn try_variant_mut(&mut self) -> Option<&mut LdtDescriptor> {
        if is_ldt_type!(self) {
            unsafe { Some(::core::mem::transmute(self)) }
        } else {
            None
        }
    }
}

/// Bitmasks of flags in Call Gate descriptor.
enum CallGateFlag {
    Dpl     = (1 << 14) | (1 << 13),
    Present = 1 << 15,
}

/// Bitmasks of flags in TSS and LDT descriptors.
enum TssLdtFlags {

    // First flags byte.
    Dpl             = (1 << 14) | (1 << 13),
    Present         = 1 << 15,

    // Second flags byte.
    Limit           = 0xF,
    Available       = 1 << 4,
    Granularity     = 1 << 7,
}

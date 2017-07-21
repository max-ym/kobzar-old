use super::{Dpl, Entry, EntryVariant};

/// General 8 byte GDT descriptor.
#[repr(packed)]
pub struct GdtDescriptor8 {
    data    : u64,
}

/// General 16 byte GDT descriptor.
#[repr(packed)]
pub struct GdtDescriptor16 {
    data    : [u64; 2],
}

/// Handle for descriptor that can either be 8 byte or 16 byte wide.
pub enum GdtDescriptorHandle<'a> {
    Null(&'a NullDescriptor),
    CallGate(&'a CallGateDescriptor),
    Tss(&'a TssDescriptor),
    Ldt(&'a LdtDescriptor),
}

impl Entry for GdtDescriptor16 {
}

impl Entry for GdtDescriptor8 {
}

impl<'a> From<&'a GdtDescriptor8> for GdtDescriptorHandle<'a> {

    fn from(r: &GdtDescriptor8) -> GdtDescriptorHandle {
        let flag_mask = 0x0F000000;

        let is_cgd_type = |data: u64| -> bool {
            use super::DescriptorType::CallGate;
            (data & flag_mask) >> 8 == CallGate as _
        };

        let is_tss_type = |data: u64| -> bool {
            use super::DescriptorType::{TssAvailable, TssBusy};
            let t = (data & flag_mask) >> 8;
            t == TssAvailable as _ || t == TssBusy as _
        };

        let is_ldt_type = |data: u64| -> bool {
            use super::DescriptorType::Ldt;
            (data & flag_mask) >> 8 == Ldt as _
        };

        let is_null_type = |data: u64| -> bool {
            (data & flag_mask) == 0
        };

        use super::GdtDescriptorHandle::*;

        unsafe {
            if is_cgd_type(r.data) {
                CallGate(::core::mem::transmute(r))
            } else if is_tss_type(r.data) {
                Tss(::core::mem::transmute(r))
            } else if is_ldt_type(r.data) {
                Ldt(::core::mem::transmute(r))
            } else if is_null_type(r.data) {
                Null(::core::mem::transmute(r))
            } else {
                unreachable!()
            }
        }
    }
}

impl Into<(GdtDescriptor8, GdtDescriptor8)> for GdtDescriptor16 {

    fn into(self) -> (GdtDescriptor8, GdtDescriptor8) {
        let a = GdtDescriptor8 { data: self.data[0] };
        let b = GdtDescriptor8 { data: self.data[1] };
        (a, b)
    }
}

/// The first descriptor in GDT is null.
#[repr(packed)]
pub struct NullDescriptor {
    null    : u64
}

impl Entry for NullDescriptor {
}

impl Default for NullDescriptor {

    fn default() -> Self {
        NullDescriptor {
            null: 0
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

/// Bitmasks of flags in Call Gate descriptor.
enum CallGateFlag {
    Dpl     = (1 << 14) | (1 << 13),
    Present = 1 << 15,
}

/// Bitmasks of flags in TSS and LDT descriptors.
enum TssLdtFlag {

    // First flags byte.
    Dpl             = (1 << 14) | (1 << 13),
    Present         = 1 << 15,

    // Second flags byte.
    Limit           = 0xF,
    Available       = 1 << 4,
    Granularity     = 1 << 7,
}

impl CallGateDescriptor {

    pub fn present(&self) -> bool {
        use self::CallGateFlag::Present;
        self.flags & Present as u16 != 0
    }

    pub fn set_present(&mut self, v: bool) {
        use self::CallGateFlag::Present;
        let mask = if v { Present as u16 } else { 0 };
        self.flags = self.flags & !(Present as u16) | mask;
    }

    pub fn dpl(&self) -> Dpl {
        use self::CallGateFlag::Dpl as DplFlag;
        let val = self.flags & DplFlag as u16;
        let val = val >> 13;
        Dpl::from_num(val as _).unwrap()
    }

    pub fn set_dpl(&mut self, dpl: Dpl) {
        use self::CallGateFlag::Dpl as DplFlag;
        let mask = (dpl as u16) << 13;
        self.flags = self.flags & !(DplFlag as u16) | mask;
    }

    pub fn offset(&self) -> u64 {
        let mut val = 0;
        val |= (self.offset0 as u64) << 00;
        val |= (self.offset1 as u64) << 16;
        val |= (self.offset2 as u64) << 32;
        val
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.offset0 = (offset >> 00) as u16;
        self.offset1 = (offset >> 16) as u16;
        self.offset2 = (offset >> 32) as u32;
    }

    pub fn segsel(&self) -> u16 {
        self.segsel
    }

    pub fn set_segsel(&mut self, segsel: u16) {
        self.segsel = segsel;
    }
}

impl TssDescriptor {

    pub fn dpl(&self) -> Dpl {
        use self::TssLdtFlag::Dpl as DplFlag;
        let val = self.flags0 & DplFlag as u16;
        let val = val >> 13;
        Dpl::from_num(val as _).unwrap()
    }

    pub fn set_dpl(&mut self, dpl: Dpl) {
        use self::TssLdtFlag::Dpl as DplFlag;
        let mask = (dpl as u16) << 13;
        self.flags0 = self.flags0 & !(DplFlag as u16) | mask;
    }

    pub fn present(&self) -> bool {
        use self::TssLdtFlag::Present;
        self.flags0 & Present as u16 != 0
    }

    pub fn set_present(&mut self, v: bool) {
        use self::TssLdtFlag::Present;
        let mask = if v { Present as u16 } else { 0 };
        self.flags0 = self.flags0 & !(Present as u16) | mask;
    }

    pub fn limit(&self) -> u8 {
        self.flags1 & 0xF
    }

    pub fn set_limit(&mut self, limit: u8) {
        use self::TssLdtFlag::Limit;
        self.flags1 &= !(Limit as u8);
        self.flags1 |= limit & Limit as u8;
    }

    pub fn available(&self) -> bool {
        use self::TssLdtFlag::Available;
        self.flags1 & (Available as u8) != 0
    }

    pub fn set_available(&mut self, avl: bool) {
        use self::TssLdtFlag::Available;
        self.flags1 &= !(Available as u8);
        self.flags1 |= if avl { Available as u8 } else { 0 };
    }

    pub fn granularity(&self) -> bool {
        self.flags1 & self::TssLdtFlag::Granularity as u8 != 0
    }

    pub fn set_granularity(&mut self, g: bool) {
        use self::TssLdtFlag::Granularity;
        self.flags1 &= !(Granularity as u8);
        self.flags1 |= if g { Granularity as u8 } else { 0 };
    }
}

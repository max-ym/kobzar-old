use super::{Entry, EntryVariant};

/// Page Table entry. Page table level 1 entry. Maps 4KiB page.
#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct P1E {
    data    : u64
}

/// Page Directory entry. Page table level 2 entry. Maps 2MiB page.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct P2EMap {
    data    : u64
}

/// Page Directory entry. Page table level 2 entry. References P1 table.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct P2ERef {
    data    : u64
}

/// Page Directory entry. Page table level 2 entry. Can be interpreted either
/// as P2EMap or P2ERef.
#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct P2E {
    data    : u64
}

/// Page Directory Pointer entry. Page table level 3 entry.
#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct P3E {
    data    : u64
}

/// Page Map Level 4 entry. Page table level 4 entry.
#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct P4E {
    data    : u64
}

impl Default for P2EMap {

    fn default() -> Self {
        P2EMap {
            data : 1 << 7, // Turn on PS
        }
    }
}

impl Default for P2ERef {

    fn default() -> Self {
        P2ERef {
            data : 0 << 7, // PS is off
        }
    }
}

impl ::core::convert::Into<P2E> for P2ERef {

    fn into(self) -> P2E {
        P2E {
            data : self.data
        }
    }
}

impl ::core::convert::Into<P2E> for P2EMap {

    fn into(self) -> P2E {
        P2E {
            data : self.data
        }
    }
}

/// Create getter and setter for single bit in page struct.
macro_rules! flag_rw {
    ($i:expr, $get_name:ident, $set_name:ident) => (
        #[inline(always)]
        fn $get_name(&self) -> bool {
            (self.data() & (1 << $i)) != 0
        }

        #[inline(always)]
        fn $set_name(&mut self, v: bool) {
            let bit = 1 << $i;
            let mask = !0u64 ^ bit;
            let d = self.data() & mask;

            let bit = if v { bit } else { 0 };

            unsafe { self.set_data(d | bit); }
        }
    )
}

/// Cancel functions created with macro 'flag_rw'.
/// Getter will always return false. Setter will do nothing.
macro_rules! flag_rw_cancel {
    ($get_name:ident, $set_name:ident) => (
        fn $get_name(&self) -> bool { false }
        fn $set_name(&mut self, _v: bool) { }
    )
}

macro_rules! fn_addr_12 {
    () => (
        fn addr(&self) -> u64 {
            let mask = 0x0007FFFFFFFFF000;
            self.data & mask
        }

        unsafe fn set_addr(&mut self, a: u64) {
            let mask = 0x0007FFFFFFFFF000;
            self.data = self.data & !mask + a;
        }
    )
}

macro_rules! fn_addr_13 {
    () => (
        fn addr(&self) -> u64 {
            let mask = 0x0007FFFFFFFFE000;
            self.data & mask
        }

        unsafe fn set_addr(&mut self, a: u64) {
            let mask = 0x0007FFFFFFFFE000;
            self.data = self.data & !mask + a;
        }
    )
}

pub trait PageFlags {

    #[inline(always)]
    fn data(&self) -> u64;

    #[inline(always)]
    unsafe fn set_data(&mut self, data: u64);

    /// Address of memory referenced by this page.
    #[inline(always)]
    fn addr(&self) -> u64;

    /// Set address of memory referenced by this page. Does not check
    /// if address is valid!
    #[inline(always)]
    unsafe fn set_addr(&mut self, a: u64);

    flag_rw!(0x00, present  , set_present   );
    flag_rw!(0x01, rw       , set_rw        );
    flag_rw!(0x02, us       , set_us        );
    flag_rw!(0x03, pwt      , set_pwt       );
    flag_rw!(0x04, pcd      , set_pcd       );
    flag_rw!(0x05, accessed , set_accessed  );
    flag_rw!(0x06, dirty    , set_dirty     );
    flag_rw!(0x07, pat      , set_pat       );
    flag_rw!(0x07, ps       , set_ps        );
    flag_rw!(0x08, global   , set_global    );
    flag_rw!(0x3F, xd       , set_xd        );
}

impl PageFlags for P1E {

    fn data(&self) -> u64 {
        self.data
    }

    unsafe fn set_data(&mut self, data: u64) {
        self.data = data;
    }

    // Not availbale for P1E.
    flag_rw_cancel!(ps, set_ps);

    fn_addr_12!();
}

impl PageFlags for P2EMap {

    fn data(&self) -> u64 {
        self.data
    }

    unsafe fn set_data(&mut self, data: u64) {
        self.data = data;
    }

    // Change bit id from 0x07 to 0x0C.
    flag_rw!(0x0C, pat, set_pat);

    fn_addr_12!();

    /// Must be 'true' to map to 2MiB page.
    fn ps(&self) -> bool { true }
    fn set_ps(&mut self, _v: bool) {}
}

impl PageFlags for P2ERef {

    fn data(&self) -> u64 {
        self.data
    }

    unsafe fn set_data(&mut self, data: u64) {
        self.data = data;
    }

    // Not exist.
    flag_rw_cancel!(pat     , set_pat       );
    flag_rw_cancel!(dirty   , set_dirty     );
    flag_rw_cancel!(global  , set_global    );

    fn_addr_13!();

    /// Must be 'false' to reference level 1 page table.
    fn ps(&self) -> bool { false }
    fn set_ps(&mut self, _v: bool) {}
}

impl PageFlags for P3E {

    fn data(&self) -> u64 {
        self.data
    }

    unsafe fn set_data(&mut self, data: u64) {
        self.data = data;
    }

    // Change bit id from 0x07 to 0x0C.
    flag_rw!(0x0C, pat, set_pat);

    fn_addr_12!();
}

impl PageFlags for P4E {

    fn data(&self) -> u64 {
        self.data
    }

    unsafe fn set_data(&mut self, data: u64) {
        self.data = data;
    }

    // Ignored.
    flag_rw_cancel!(dirty   , set_dirty );
    flag_rw_cancel!(global  , set_global);

    fn_addr_12!();
}

impl Entry for P1E {
}

impl Entry for P2E {
}

impl Entry for P3E {
}

impl Entry for P4E {
}

impl Entry for P2EMap {
}

impl Entry for P2ERef {
}

impl EntryVariant<P2EMap> for P2E {

    fn try_variant_ref(&self) -> Option<&P2EMap> {
        let entry = unsafe { &*(self as *const _ as *mut P2EMap) };
        if entry.ps() {
            None
        } else {
            Some(entry)
        }
    }

    fn try_variant_mut(&mut self) -> Option<&mut P2EMap> {
        let entry = unsafe { &mut *(self as *const _ as *mut P2EMap) };
        if entry.ps() {
            None
        } else {
            Some(entry)
        }
    }
}

impl EntryVariant<P2ERef> for P2E {

    fn try_variant_ref(&self) -> Option<&P2ERef> {
        let entry = unsafe { &*(self as *const _ as *mut P2ERef) };
        if entry.ps() {
            Some(entry)
        } else {
            None
        }
    }

    fn try_variant_mut(&mut self) -> Option<&mut P2ERef> {
        let entry = unsafe { &mut *(self as *const _ as *mut P2ERef) };
        if entry.ps() {
            Some(entry)
        } else {
            None
        }
    }
}

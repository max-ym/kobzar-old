/// Page Table entry. Page table level 1 entry. Maps 4KiB page.
#[repr(packed)]
pub struct P1E {
    data    : u64
}

/// Page Directory entry. Page table level 2 entry. Maps 2MiB page.
#[repr(packed)]
pub struct P2EMap {
    data    : u64
}

/// Page Directory entry. Page table level 2 entry. References P1 table.
#[repr(packed)]
pub struct P2ERef {
    data    : u64
}

/// Page Directory Pointer entry. Page table level 3 entry.
#[repr(packed)]
pub struct P3E {
    data    : u64
}

/// Page Map Level 4 entry. Page table level 4 entry.
#[repr(packed)]
pub struct P4E {
    data    : u64
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
        fn $set_name(&mut self, v: bool) { }
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

    /// Set address of memory referenced by this page.
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
    fn set_ps(&mut self, v: bool) {}
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
    fn set_ps(&mut self, v: bool) {}
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

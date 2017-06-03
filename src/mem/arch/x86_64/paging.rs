use arch::paging::*;
use mem::map::{PAGING_P4, PAGING_P3, PAGING_P2, PAGING_P1};

fn p1() -> &'static mut P1 {
    unsafe {
        let a: *mut P1 = ::core::mem::transmute(PAGING_P1);
        &mut *a
    }
}

fn p2() -> &'static mut P2 {
    unsafe {
        let a: *mut P2 = ::core::mem::transmute(PAGING_P2);
        &mut *a
    }
}

fn p3() -> &'static mut P3 {
    unsafe {
        let a: *mut P3 = ::core::mem::transmute(PAGING_P3);
        &mut *a
    }
}

fn p4() -> &'static mut P4 {
    unsafe {
        let a: *mut P4 = ::core::mem::transmute(PAGING_P4);
        &mut *a
    }
}

/// Initialize and load kernel paging table.
pub fn setup() {
    use arch::tentr::*;

    unsafe {
        use core::intrinsics::write_bytes;
        // Set all pages in tables to zero.
        write_bytes(p1() as *const _ as *mut P1, 0, 1);
        write_bytes(p2() as *const _ as *mut P2, 0, 1);
        write_bytes(p3() as *const _ as *mut P3, 0, 1);
        write_bytes(p4() as *const _ as *mut P4, 0, 1);
    }

    unsafe {
        // Setup P4 entry. This entry covers the first 512 GiB of RAM.
        let p4e = p4().entry_mut(0);

        p4e.set_rw(true); // Readable and Writable.

        // NOT accessible for user-mode processes.
        // This page table must be used only by the kernel. No
        // user-space process must never use this table. So
        // this flag must not be set.
        p4e.set_us(false);

        p4e.set_addr(p3() as *const _ as u64); // P3 table address.

        p4e.set_present(true);
    }

    unsafe {
        // Setup P3 entries.
        // First entry covers first 1GiB of RAM.
        let p3e = p3().entry_mut(0);
        p3e.set_rw(true);
        p3e.set_us(false);
        p3e.set_addr(p2() as *const _ as u64);
        p3e.set_present(true);
    }

    unsafe {
        // Each P2 entry covers 2MiB region.

        let mut p2e = P2ERef::default();
        p2e.set_rw(true);
        p2e.set_us(false);
        p2e.set_addr(p1() as *const _ as u64);
        p2e.set_present(true);

        *p2().entry_mut(0) = p2e.into();
    }

    unimplemented!()
}

use arch::tables::paging::*;
use mem::map::{PAGING_P4, PAGING_P3, PAGING_P2, PAGING_P1};

/// Get reference to P1 page struct of the kernel.
fn p1() -> &'static mut P1 {
    unsafe {
        let a: *mut P1 = ::core::mem::transmute(PAGING_P1);
        &mut *a
    }
}

/// Get reference to P2 page struct of the kernel.
fn p2() -> &'static mut P2 {
    unsafe {
        let a: *mut P2 = ::core::mem::transmute(PAGING_P2);
        &mut *a
    }
}

/// Get reference to P3 page struct of the kernel.
fn p3() -> &'static mut P3 {
    unsafe {
        let a: *mut P3 = ::core::mem::transmute(PAGING_P3);
        &mut *a
    }
}

/// Get reference to P4 page struct of the kernel.
fn p4() -> &'static mut P4 {
    unsafe {
        let a: *mut P4 = ::core::mem::transmute(PAGING_P4);
        &mut *a
    }
}

/// Initialize and load kernel paging table. Creates regions for
/// normal data and code. Also, disables cache for regions with
/// mapped I/O devices.
pub fn setup() {
    unsafe {
        use core::intrinsics::write_bytes;
        // Set all pages in tables to zero.
        write_bytes(p1() as *const _ as *mut P1, 0, 1);
        write_bytes(p2() as *const _ as *mut P2, 0, 1);
        write_bytes(p3() as *const _ as *mut P3, 0, 1);
        write_bytes(p4() as *const _ as *mut P4, 0, 1);
    }

    use arch::tables::*;

    unsafe {
        // Setup P4 entry. This entry covers the first 512 GiB of RAM.
        let p4e = p4().entry_handle(0).unwrap().variant();
        let p4e = match p4e { P4EVariant::P4E(a) => a };

        let flags =
            PageFlag::rw        () | // Readable and Writable.
            PageFlag::present   () ;

        // US flag is off.
        // NOT accessible for user-mode processes.
        // This page table must be used only by the kernel. No
        // user-space process must never use this table. So
        // this flag must not be set.

        let addr = p3() as *const _ as u64;
        let b: u64 = PageFlag::p4addr().into();
        if addr != addr & b {
            panic!("Address invalid");
        }

        p4e.data_rewrite(flags | PageFlag::from(addr));
    }

    unsafe {
        // Setup P3 entries.
        // First entry covers first 1GiB of RAM.
        let p3e = p3().entry_handle(0).unwrap().variant();
        let p3e = match p3e { P3EVariant::P3E(a) => a };

        let flags =
            PageFlag::rw        () | // Readable and Writable.
            PageFlag::present   () ;

        let addr = PageFlag::from(p2() as *const _ as u64);

        p3e.data_rewrite(flags | addr);
    }

    unsafe {
        // Each P2 entry covers 2MiB region.
        let p2e = p2().entry_handle(0).unwrap();

        let mut val = P2ERef::default();

        let flags =
            PageFlag::rw        () |
            PageFlag::present   () ;

        let addr = PageFlag::from(p1() as *const _ as u64);

        val.data_rewrite(flags | addr);

        p2e.set_ref(val);
    }

    unsafe {
        // 0x00000 - 0x00FFF
        let p = match p1().entry_handle(0).unwrap().variant() {
            P1EVariant::P1E(a) => a
        };

        let flags =
            PageFlag::rw        () |
            PageFlag::present   () ;

        let addr = PageFlag::from(0x00000);

        p.data_rewrite(flags | addr);

        // 0x01000 - 0x01FFF: APIC registers.
        // Assertion fail when memory map was changed by someone.
        // Code below must be reviewed in such a case and changed too.
        assert!(super::map::APIC_BASE_ADDRESS == 0x01000);
        let p = match p1().entry_handle(1).unwrap().variant() {
            P1EVariant::P1E(a) => a
        };

        let flags =
            PageFlag::rw        () |
            PageFlag::pwt       () | // Write-through.
            PageFlag::pcd       () | // Disbale caching.
            PageFlag::present   () ;

        let addr = PageFlag::from(0x01000);

        p.data_rewrite(flags | addr);

        let flags =
            PageFlag::rw        () |
            PageFlag::present   () ;

        // Conventional memory.
        for i in 0x02..0x9F {
            let p = match p1().entry_handle(i).unwrap().variant() {
                P1EVariant::P1E(a) => a
            };

            let addr = PageFlag::from(0x1000 * i as u64);

            p.data_rewrite(flags | addr);
        }

        let flags =
            PageFlag::rw        () |
            PageFlag::pwt       () |
            PageFlag::pcd       () |
            PageFlag::present   () ;

        // I/O devices are mapped in this region.
        for i in 0xA0..0xFF {
            let p = match p1().entry_handle(i).unwrap().variant() {
                P1EVariant::P1E(a) => a
            };

            let addr = PageFlag::from(0x1000 * i as u64);

            p.data_rewrite(flags | addr);
        }

        let flags =
            PageFlag::rw        () |
            PageFlag::present   () ;

        // Map second megabyte where OS code is stored.
        for i in 0x100..0x200 {
            let p = match p1().entry_handle(i).unwrap().variant() {
                P1EVariant::P1E(a) => a
            };

            let addr: PageFlag = (0x1000 * i as u64).into();

            p.data_rewrite(flags | addr);
        }
    }

    // Save P4 address to CR3 and so start using new paging.
    unsafe {
        use arch::cr::{Cr3, Reg};
        let mut cr3 = Cr3::read();
        cr3.set_addr(p4() as *const _ as _);
        cr3.save();
    }
}

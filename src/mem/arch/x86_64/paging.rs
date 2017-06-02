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

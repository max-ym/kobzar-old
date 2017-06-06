/// Paging tables.
pub mod paging;

/// Module for structs of register files.
pub mod regf;

/// Code related to using I/O APIC and Local APIC.
pub mod apic;

/// Descriptor Table module. Contains IDT, GDT.
mod dt;
pub use self::dt::*;

/// Table Entries traits. Used for system tables like GDT, IDT,
/// Paging tables etc.
pub mod tentr;

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: u8, n: usize) -> *mut u8 {
    let mut dest    = dest;
    let     res     = dest.clone();
    let mut n       = n as usize;

    if n == 0 {
        return res;
    }

    // Get 8-byte pattern.
    let c = {
        let c = c as u64;
        let a = c << 8;
        let c = c | a;

        let a = c << 16;
        let c = c | a;

        let a = c << 32;
        c | a
    };

    /// Fill 1 more byte of memory.
    unsafe fn fill1(dest: &mut *mut u8, c: u64, n: &mut usize) {
        **dest = c as u8;
        *dest = (*dest).offset(1);
        *n = *n - 1;
    };

    /// Align to 2-byte boundary.
    unsafe fn align2(dest: &mut *mut u8, c: u64, n: &mut usize) {
        let dest_addr = *dest as usize;
        if dest_addr % 2 != 0 {
            fill1(dest, c, n);
        }
    };

    /// Fill 2 more bytes of memory.
    unsafe fn fill2(mut count: usize, dest: &mut *mut u8, c: u64, n: &mut usize) {
        *n = *n - count * 2;
        while count > 0 {
            let d = *dest as *mut u16;
            *d = c as u16;
            *dest = (*dest).offset(2);
            count = count - 1;
        }
    };

    /// Fill 8 more byte of memory.
    unsafe fn fill8(mut count: usize, dest: &mut *mut u8, c: u64, n: &mut usize) {
        *n = *n - count * 8;
        while count > 0 {
            let d = *dest as *mut u64;
            *d = c;
            *dest = (*dest).offset(8);
            count = count - 1;
        }
    };

    if n == 2 {
        fill2(1, &mut dest, c, &mut n);
        return res;
    }
    align2(             &mut dest, c, &mut n);
    fill2((n %  8) / 2, &mut dest, c, &mut n);
    fill8((n % 32) / 8, &mut dest, c, &mut n);
    fill2((n % 32) / 2, &mut dest, c, &mut n);
    align2(             &mut dest, c, &mut n);

    if n > 32 {
        asm!(
            "rep stosq"
        :
        :   "{rax}" (c), "{rcx}" (n / 8), "{rdi}" (dest as usize)
        :   "rcx", "rdi"
        :   "volatile"
        );
    }

    // Fill the end which gone out of 32-byte boundary.
    fill8(n / 8, &mut dest, c, &mut n);
    fill2(n / 2, &mut dest, c, &mut n);
    if n == 1 {
        fill1(&mut dest, c, &mut n);
    }

    res
}

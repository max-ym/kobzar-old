#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]

#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]

#![allow(dead_code)]

// Contains some functions that perform same operations that otherwise would
// require using asm! macro.
#[cfg(target_arch = "x86_64")]
extern crate asm_x86_64;

#[cfg(target_arch = "x86_64")]
use asm_x86_64 as asm;

/// All the stuff that is needed at early initialization.
mod early;

/// Module to work with physical memory: memory protection mechanisms, paging
/// and other related stuff is located here.
mod mem;

/// Module to handle CCS networking of local objects (programs and threads).
mod ccs;

/// Architecture-dependent code. Defines architectural structs and functions
/// but does not contain the code which directly accesses this stuff for
/// implementing anything inside kernel. Only general things are kept in this
/// module.
///
/// TODO: consider moving this module to seperate library later.
mod arch;

/// The starting point of kernel Rust code execution.
/// Before this point runs some initial assembly code that initializes
/// the environment where Rust code can start performing.
#[no_mangle]
pub extern fn main() -> ! {
    /* Things to be done:
     *
     * Initialize CCS model.
     * Start multi-threading.
     * Load memory manager.
     *
     * Start initial kernel graphics server.
     * Start initial kernel logger.
     * Start file system servers.
     * Run startup script (if any is found).
     * Start kernel terminal.
     */

     // Start the very first logger and display driver.
    use early::{LoggerTrait, logger};
    logger().println("Kobzar kernel logger greets you!");
    logger().println("Very first initialization begins! Hold on tight ^-^\n");

    // Setup paging first to enable caching and correct communication
    // with memory mapped devices.
    logger().println("Enabling new initial kernel paging tables.");
    ::mem::paging::setup();

    logger().println("Setting up basic CCS table.");
    ::ccs::setup();

    //setup_interrupts();

    halt_forever();
}

/// Setup mechanisms that are controlling the interrupts.
#[cfg(target_arch = "x86_64")]
fn setup_interrupts() {
    ::early::setup_interrupts();
}

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn memset_avx(dest: *mut u8, c: u64, n: usize);
}

#[no_mangle]
#[cfg(target_arch = "x86_64")]
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

    unsafe fn fill1(dest: &mut *mut u8, c: u64, n: &mut usize) {
        **dest = c as u8;
        *dest = (*dest).offset(1);
        *n = *n - 1;
    };

    unsafe fn align2(dest: &mut *mut u8, c: u64, n: &mut usize) {
        let dest_addr = *dest as usize;
        if dest_addr % 2 != 0 {
            // Align to 2-byte boundary.
            fill1(dest, c, n);
        }
    };

    unsafe fn fill2(mut count: usize, dest: &mut *mut u8, c: u64, n: &mut usize) {
        *n = *n - count * 2;
        while count > 0 {
            let d = *dest as *mut u16;
            *d = c as u16;
            *dest = (*dest).offset(2);
            count = count - 1;
        }
    };

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
    align2(&mut dest, c, &mut n);
    fill2((n %  8) / 2, &mut dest, c, &mut n);
    fill8((n % 32) / 8, &mut dest, c, &mut n);
    fill2((n % 32) / 2, &mut dest, c, &mut n);
    align2(&mut dest, c, &mut n);

    if n > 32 {
        // TODO support processors with no AVX.
        memset_avx(dest, c, n / 32);
    }

    // Fill the end which gone out of 32-byte boundary.
    fill8(n / 8, &mut dest, c, &mut n);
    fill2(n / 2, &mut dest, c, &mut n);
    if n == 1 {
        fill1(&mut dest, c, &mut n);
    }

    res
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32) -> ! {
    use early::{logger};
    use core::fmt::Write;

    write!(logger(), "PANIC! in file '{}', line '{}'", file, line).unwrap();
    logger().write_fmt(msg).unwrap();

    halt_forever();
}

#[cfg(target_arch = "x86_64")]
fn halt_forever() -> ! {
    loop { unsafe { asm!("cli \n hlt"); }}
}

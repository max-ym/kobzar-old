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

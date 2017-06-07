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

// Export global function 'memset'.
pub use arch::memset;

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

    halt_forever();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
    halt_forever();
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_impl(_fmt: ::core::fmt::Arguments,
                        _file: &'static str, _line: u32) -> ! {
    halt_forever();
}

#[cfg(target_arch = "x86_64")]
fn halt_forever() -> ! {
    loop { unsafe { asm!("cli \n hlt"); }}
}

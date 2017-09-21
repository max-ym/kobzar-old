#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]

#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]

#![allow(dead_code)]

#[cfg(target_arch = "x86_64")]
extern crate new_bitflags;

// Contains some functions that perform same operations that otherwise would
// require usage of asm! macro.
// Also contain modules related to architecture.
#[cfg(target_arch = "x86_64")]
extern crate asm_x86_64;

#[cfg(target_arch = "x86_64")]
use asm_x86_64 as arch;

/// All the stuff that is needed at early initialization.
mod early;

/// Module to work with physical memory: memory protection mechanisms, paging
/// and other related stuff is located here.
mod mem;

/// Module to handle CCS networking of local objects (programs and threads).
mod ccs;

// Export global function 'memset'.
// pub use arch::memset;

/// Interrupt-handling module.
mod ints;

/// Module to provide system timer implementation.
mod timer;

macro_rules! panic {
    () => {{
        use early::logger;
        use core::fmt::Write;
        write!(logger(),
            "PANIC! in file {}: line {}\n", file!(), line!()).unwrap();
    }};

    ($msg:expr) => {{
        use early::logger;
        use core::fmt::Write;
        panic!();
        logger().println($msg);
    }};

    ($fmt:expr, $($arg:tt)+) => {{
        use early::logger;
        use core::fmt::Write;
        panic!();
        write!(logger(), $fmt, $arg).unwrap();
    }};
}

/// The starting point of kernel Rust code execution.
/// Before this point runs some initial assembly code that initializes
/// the environment where Rust code can start performing.
#[no_mangle]
pub extern fn main() -> ! {
    /* Things to be done:
     *
     * Setup proper paging.
     * Initialize CCS model.
     * Set proper interrupt-handling system.
     * Start scheduler and multi-threading.
     *
     * Start memory manager server.
     * Start basic kernel graphics server.
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
    //::mem::paging::setup();

    logger().println("Setting up basic CCS table.");
    //::ccs::setup();

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

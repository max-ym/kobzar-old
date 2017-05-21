#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]

#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]

/// All the stuff that is needed at early initialization.
mod early;

/// Interrupt handling.
mod interrupts;

/// Some functions that perform same operations that otherwise would
/// require using asm! macro.
mod asm;

/// Module to work with physical memory: memory protection mechanisms, paging
/// and other related stuff is located here.
mod mem;

/// Module to handle CCS networking of local objects (programs and threads).
mod ccs;

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
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    halt_forever();
}

#[cfg(target_arch = "x86_64")]
fn halt_forever() -> ! {
    loop { unsafe { asm!("cli \n hlt"); }}
}

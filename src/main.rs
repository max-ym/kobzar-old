#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]

#![feature(lang_items)]
#![feature(asm)]

/// All the stuff that is needed at early initialization.
mod early;

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
    use early::LoggerTrait;
    let mut logger = start_stupid_logger();
    logger.println("Kobzar kernel logger greets you!");
    logger.println("Very first initialization begins! Hold on tight ^-^");

    halt_forever();
}

#[cfg(target_arch = "x86_64")]
fn start_stupid_logger() -> ::early::Logger {
    ::early::Logger::new()
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

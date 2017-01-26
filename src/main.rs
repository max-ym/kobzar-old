#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]

#![feature(lang_items)]
#![feature(asm)]

/// The starting point of kernel Rust code execution.
/// Before this point runs some initial assembly code that initializes
/// the environment where Rust code can start performing.
#[no_mangle]
#[lang = "start"]
pub fn main() -> ! {
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

    loop {};
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
    unsafe { asm!("cli \n hlt"); }
    loop {}
}

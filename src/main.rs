#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![no_std]

#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(untagged_unions)]

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
/// and other related stuff is located here. Also contains
/// architecture-independent traits of memory allocators.
mod mem;

/// Module to handle CCS networking of local objects (programs and threads).
/// Implements process management and scheduling.
mod ccs;

/// Interrupt-handling module.
mod ints;

/// Module to provide system timer implementation.
mod timer;

mod collections;

// /// Collections that are abscent in 'core' crate but are useful.
// /// When there is a need to store the data collection in a way that
// /// was yet not defined in this module consider implementing it
// /// and only then switching to using it in a destination module that
// /// required this new implementation.
// mod collections;

/// The starting point of kernel Rust code execution.
/// Earlier was launched the initial assembly code that initialized
/// the environment where Rust code could perform.
#[no_mangle]
pub extern fn main() -> ! {
    halt_forever();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
    halt_forever();
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_fmt: ::core::fmt::Arguments,
                        _file: &'static str, _line: u32) -> ! {
    halt_forever();
}

/// Halt the kernel process forever. Note that this may not halt all
/// cores immediately on multicore systems. It is designed to
/// be used on early system setup when unrecoverable error
/// occurs. It is expeted that system has no more than one
/// thread running.
#[cfg(target_arch = "x86_64")]
fn halt_forever() -> ! {
    loop { unsafe { asm!("cli \n hlt"); }}
}

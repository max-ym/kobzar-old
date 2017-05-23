/// The early basic logger for the system initialization process.
static mut LOGGER: Logger = Logger::new();

/// Get system early logger.
#[inline(always)]
pub fn logger() -> &'static mut Logger {
    unsafe { &mut LOGGER }
}

/// Very simple logger that is capable of logging early startup info.
pub trait LoggerTrait {

    fn print(&mut self, s: &str);

    fn println(&mut self, s: &str) {
        self.print(s);
        self.newline();
    }

    fn newline(&mut self);
}

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

/// Stuff implemented for x86_64.
#[cfg(target_arch = "x86_64")]
mod x86_64;

pub mod ccs;

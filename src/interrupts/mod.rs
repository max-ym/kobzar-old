#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

/// Interrupt handling for x86_64 architecture.
#[cfg(target_arch = "x86_64")]
mod x86_64;

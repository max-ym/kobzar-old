mod arch;
pub use self::arch::*;

/// Module for Address struct implementation. Address structure is a wrapper
/// for storing any sort of address as a number instead of a pointer on a
/// type instance.
mod addr;

pub use self::addr::Address;

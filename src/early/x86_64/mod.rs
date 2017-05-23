use super::LoggerTrait;

/// Simple logger module for early logs.
mod logger;
pub use self::logger::*;

/// Interrupts setup.
mod interrupts;
pub use self::interrupts::*;

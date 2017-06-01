/// Paging tables.
pub mod paging;

/// Module for structs of register files.
pub mod regf;

/// Code related to using I/O APIC and Local APIC.
pub mod apic;

/// Descriptor Table module. Contains IDT, GDT.
mod dt;
pub use self::dt::*;

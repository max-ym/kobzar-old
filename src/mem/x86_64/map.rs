//! Memory map after launching the kernel.
//! 00000:003FF - free
//! 00400:004FF - BIOS Data Area
//! 00500:7FFFF - free
//! >=80000 - may not all be used.
//!
//! Memory map of the kernel in selected region:
//! 00000:003FF - free
//! 00400:004FF - BIOS Data Area
//! 00500:014FF - Local APIC registers
//! 01500:01600 - CCS basic setup temporary memory.
//! 01600:7FFFF - free

/// Local APIC base registers address. They are moved from their default
/// location here. Note that the registers are 4 KiB in size.
pub const APIC_BASE_ADDRESS: u64 = 0x00500;

/// The memory used to store information about basic kernel CCS objects and
/// services when kernel starts up.
pub const CCS_BASIC_SETUP_ADDRESS: u64 = 0x01500;

/// The bound of the memory that can be used by CCS basic setup data.
pub const CCS_BASIC_SETUP_ADDRESS_END: u64 = 0x01600;

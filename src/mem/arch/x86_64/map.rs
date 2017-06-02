//! Memory map after launching the kernel.
//! 00000:003FF - free
//! 00400:004FF - BIOS Data Area.
//! 00500:7FFFF - free
//! >=80000 - may not all be used.
//!
//! Memory map of the kernel in selected region:
//! 00000:003FF - free
//! 00400:004FF - BIOS Data Area.
//! 00500:014FF - Local APIC registers.
//! 01500:015FF - CCS basic setup temporary memory.
//! 01600:01FFF - free
//! 02000:02FFF - Paging. Page Table Level 1.
//! 03000:03FFF - Paging. Page Table Level 2.
//! 04000:04FFF - Paging. Page Table Level 3.
//! 05000:05FFF - Paging. Page Table Level 4.
//! 06000:7FFFF - free

/// Local APIC base registers address. They are moved from their default
/// location here. Note that the registers are 4 KiB in size.
pub const APIC_BASE_ADDRESS: u64 = 0x00500;

/// The memory used to store information about basic kernel CCS objects and
/// services when kernel starts up.
pub const CCS_BASIC_SETUP_ADDRESS: usize = 0x01500;

/// The bound of the memory that can be used by CCS basic setup data.
pub const CCS_BASIC_SETUP_ADDRESS_END: usize = 0x015FF;

pub const PAGING_P1: usize = 0x2000;
pub const PAGING_P2: usize = 0x3000;
pub const PAGING_P3: usize = 0x4000;
pub const PAGING_P4: usize = 0x5000;

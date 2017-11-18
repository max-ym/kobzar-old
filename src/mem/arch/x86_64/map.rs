//! Memory map after launching the kernel.
//! 00000:003FF - free
//! 00400:004FF - BIOS Data Area.
//! 00500:7BFFF - free
//! 7C000:7CFFF - Initial paging. Page Table Level 2.
//! 7D000:7DFFF - Initial paging. Page Table Level 3.
//! 7E000:7EFFF - Initial paging. Page Table Level 4.
//! 7F000:7FFFF - Stack.
//! 80000:9FBFF - free but may not all be used.
//! 9FC00:9FFFF - typical location for EBDA. But no guarantees.
//! A0000:FFFFF - Video + ROM. Must not be cached, write through.
//!
//! Memory map of the kernel in selected region:
//! 00000:003FF - free
//! 00400:004FF - BIOS Data Area.
//! 00500:00FFF - CCS basic setup temporary memory.
//! 01000:01FFF - Local APIC registers.
//! 02000:02FFF - Paging. Page Table Level 1.
//! 03000:03FFF - Paging. Page Table Level 2.
//! 04000:04FFF - Paging. Page Table Level 3.
//! 05000:05FFF - Paging. Page Table Level 4.
//! 06000:06FFF - IDT.
//! 07000:08FFF - GDT.
//! 09000:7BFFF - free
//! 7C000:7CFFF - Initial paging. Page Table Level 2 OR allocator memory.
//! 7D000:7DFFF - Initial paging. Page Table Level 3 OR allocator memory.
//! 7E000:7EFFF - Initial paging. Page Table Level 4 OR allocator memory.
//! 7F000:7FFFF - Stack.
//!
//! Note that as soon as main paging tables are set,
//! memory region of 7C000:7EFFF gets free and is used by kernel memory
//! allocators.

/// Local APIC base registers address. They are moved from their default
/// location here. Note that the registers are 4 KiB in size.
pub const APIC_BASE_ADDRESS: u64 = 0x01000;

/// The memory used to store information about basic kernel CCS objects and
/// services when kernel starts up.
pub const CCS_BASIC_SETUP_ADDRESS: usize = 0x00500;

/// The bound of the memory that can be used by CCS basic setup data.
pub const CCS_BASIC_SETUP_ADDRESS_END: usize = 0x00FFF;

/// Address of the level 1 paging table of the kernel.
pub const PAGING_P1: usize = 0x2000;

/// Address of the level 2 paging table of the kernel.
pub const PAGING_P2: usize = 0x3000;

/// Address of the level 3 paging table of the kernel.
pub const PAGING_P3: usize = 0x4000;

/// Address of the level 4 paging table of the kernel.
pub const PAGING_P4: usize = 0x5000;

/// Address of Interrupt Descriptor Table.
pub const IDT: usize = 0x6000;

/// Address of Global Descriptor Table.
pub const GDT: usize = 0x7000;

/// Start of kernel memory allocator.
pub const MEMALLOC_START: usize = 0x7C000;

/// End of kernel memory allocator (excluding byte at this address).
pub const MEMALLOC_END: usize = 0x7F000;

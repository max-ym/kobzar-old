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
//! 00500:00FFF - free
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
//! Virtual memory of the kernel of above listed memory ranges is
//! directly mapped to the same addresses of physical memory.
//! Below is located virtual kernel memory which is not mapped to the
//! same physical addresses. Their physical location is defined at runtime
//! by memory allocator and may not be the same on different machines.
//!
//! Kernel virtual memory map:
//! 0x40000000:0x7FFFFFFF - Memory allocator for CCS module. (1)
//! 0x80000000:0xAFFFFFFF - Memory allocator for memory module. (1)
//!
//! Note that as soon as main paging tables are set,
//! memory region of 7C000:7EFFF gets free and is used by kernel memory
//! allocators.
//!
//! 1 - Memory allocators may not access all the memory of the range they are
//! provided with. They can only access it when particular memory was allocated
//! by
//! page allocator and must be released as soon as it is not needed anymore.

use super::Address;

macro_rules! addr {
    ($name:ident, $addr:expr, $docs:expr) => {
        #[doc=$docs]
        pub const $name: Address = Address::new_from_usize($addr as usize);
    };

    ($name:ident, $addr:expr) => {
        addr!($name, $addr, "");
    };
}

addr!(APIC_BASE, 0x1000, "
    Local APIC base registers address. They are moved from their default
    location here. Note that the registers are 4 KiB in size.
");

addr!(PAGING_P1, 0x2000, "
    Address of the level 1 paging table of the kernel.
");

addr!(PAGING_P2, 0x3000, "
    Address of the level 2 paging table of the kernel.
");

addr!(PAGING_P3, 0x4000, "
    Address of the level 3 paging table of the kernel.
");

addr!(PAGING_P4, 0x5000, "
    Address of the level 4 paging table of the kernel.
");

addr!(IDT, 0x6000, "
    Address of Interrupt Descriptor Table.
");

addr!(GDT, 0x7000, "
    Address of Global Descriptor Table.
");

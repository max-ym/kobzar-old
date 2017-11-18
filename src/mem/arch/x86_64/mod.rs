// The memory map of kernel image. Module doc is placed in 'map.rs' sources.
pub mod map;

/// Memory allocator for system. Manages pages of memory for further use.
pub mod alloc;

/// Memory pages of the kernel.
pub mod paging;

/// Global Descriptor Table of the kernel.
pub mod gdt;

use super::TopLimitedAllocator;

/// Main kernel memory allocator.
/// Is allowed to be used only when kernel paging was re-initialized.
static mut MAIN_ALLOC: TopLimitedAllocator = new_main_alloc();

const fn new_main_alloc() -> TopLimitedAllocator {
    use super::super::Address;

    TopLimitedAllocator::new(Address::new_from_usize(map::MEMALLOC_START),
            Address::new_from_usize(map::MEMALLOC_END))
}

/// Main kernel memory allocator reference.
/// Is allowed to be used only when kernel paging was re-initialized.
pub fn main_alloc() -> &'static TopLimitedAllocator {
    unsafe { &MAIN_ALLOC }
}

pub fn main_alloc_mut() -> &'static mut TopLimitedAllocator {
    unsafe { &mut MAIN_ALLOC }
}

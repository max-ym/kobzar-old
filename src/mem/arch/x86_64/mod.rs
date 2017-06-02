// The memory map of kernel image. Module doc is placed in 'map.rs' sources.
pub mod map;

/// Memory allocator for system. Manages pages of memory for further use.
pub mod alloc;

/// Memory pages of the kernel.
pub mod paging;

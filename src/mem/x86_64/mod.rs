/// Global Descriptor Table related stuff.
pub mod gdt;

// The memory map of kernel image. Module doc is placed in 'map.rs' sources.
pub mod map;

/// Memory allocator for system. Manages pages of memory for further use.
pub mod alloc;

/// Paging tables.
pub mod table;

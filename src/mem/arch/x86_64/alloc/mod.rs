/// Allocator for 2MiB pages.
pub mod p2m;

use self::p2m::Page2m;

/// Allocator for 4KiB pages.
pub mod p4k;

/// Page Status Object module.
pub mod pso;

/// Page Map Heap module.
pub mod map_heap;

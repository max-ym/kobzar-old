/// Allocator for 2MiB pages.
pub mod p2m;

pub use self::p2m::Page2m;
pub use self::p2m::Stack as Stack2m;

/// Allocator for 4KiB pages.
pub mod p4k;

pub use self::p4k::Page4k;

/// Page Status Object module.
pub mod pso;

pub use self::pso::PageStatus;
pub use self::pso::PsArray;
pub use self::pso::PsaArray;

/// Page Map Heap module.
pub mod map_heap;

pub use self::map_heap::RelativeAddress;
pub use self::map_heap::HeapEntry as Heap4kEntry;

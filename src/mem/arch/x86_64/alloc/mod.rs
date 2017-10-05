/// Structures related to 2MiB pages.
pub mod p2m;

/// Structures related to 4KiB pages.
pub mod p4k;

/// Page Status Object module.
pub mod pso;

/// Page Map Heap module.
pub mod map_heap;

pub use self::p2m::Page2m;
pub use self::p2m::Stack as Stack2m;

pub use self::p4k::Page4k;

pub use self::pso::PageStatus;
pub use self::pso::PsArray;
pub use self::pso::PsaArray;

pub use self::map_heap::RelativeAddress;
pub use self::map_heap::HeapEntry as Heap4kEntry;

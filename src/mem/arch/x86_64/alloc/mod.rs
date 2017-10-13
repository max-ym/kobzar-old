/// Main controller that uses all submodules to provide interface for
/// allocating and releasing pages of memory.
pub mod ctrl;

/// Structures related to 2MiB pages.
pub mod p2m;

/// Structures related to 4KiB pages.
pub mod p4k;

/// Page Status Object module. Page Status holds information that is used
/// in page allocator. For example, it holds a counter of how many page
/// table entries use this page. This allows allocator to know when page
/// gets free from users and whether it can be safely allocated for
/// new processes.
pub mod pso;

/// Page Map Heap module. Heap stores Page Status entries for each 4KiB page
/// that were created by dividing 2MiB pages.
pub mod map_heap;

pub use self::p2m::Page2m;
pub use self::p2m::Range as Range2m;
pub use self::p2m::Stack as Stack2m;

pub use self::p4k::Page4k;

pub use self::pso::PageStatus;
pub use self::pso::PsArray;
pub use self::pso::PsaArray;

pub use self::map_heap::RelativeAddress;
pub use self::map_heap::HeapEntry as Heap4kEntry;

use super::*;

// The memory map of kernel image. Module doc is placed in 'map.rs' sources.
pub mod map;

/// Memory manager. Implements a system object that provides services of
/// memory allocation.
pub mod mgr;

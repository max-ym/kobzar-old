/// Allocator for 2MiB pages.
pub mod p2m;

use self::p2m::{Page2m, Page2mStatus};

/// Allocator for 4KiB pages.
pub mod p4k;

/// Page Status Object module.
pub mod pso;

/// Allocator for 2MiB pages.
pub mod p2m;

use self::p2m::Page2m;

/// Allocator for 4KiB pages.
pub mod p4k;

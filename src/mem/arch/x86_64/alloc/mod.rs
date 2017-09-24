/// Allocator for 2MiB pages.
pub mod p2m;

use self::p2m::Page2m;

/// Allocator for 4KiB pages.
pub mod p4k;

/// B-Tree structure that stores all usable 2MiB pages and their status.
pub mod btree;

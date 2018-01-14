use super::ProcessCount;
use collections::{
    FixedArray,
    Bitmap512,
};
use mem::Address;

/// The page tree master structure.
struct Tree {
    middle_addr : Address,
    lower       : FixedArray<NodeData>,
    upper       : FixedArray<NodeData>,
}

/// The data of a single tree node.
struct NodeData {
    base_addr   : Address,
    arr         : FixedArray<ArrayEntry>,
}

struct ArrayEntry {
    is_divided  : bool,
    data        : PageDataUnion,
}

/// Union that contains either page division data address OR allocation count.
/// This depends whether the page was divided into smaller or was allocated
/// as single.
union PageDataUnion {
    div_data    : *mut DivData,
    alloc_count : ProcessCount,
}

/// Data about each 4 KiB page that was formed by dividing 2 MiB page into
/// 512 4 KiB pages.
struct DivData {

    /// Map that indicates the
    map     : Bitmap512,

    /// Array of counters for each page. Counter of page allocation.
    counters: [ProcessCount; 512],
}

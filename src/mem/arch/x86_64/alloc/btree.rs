use super::p2m::Page2mStatus;

/// B-Tree of 2MiB pages.
pub struct PageBTree {
}

pub struct BTreeNode {
}

/// Leaf of B-tree. It was calculated that optimal leaf element count is 4.
/// So each leaf stores 4 pointers to page status structures in the heap.
#[repr(packed)]
pub struct BTreeLeaf {
    arr     : [*mut Page2mStatus; 4]
}

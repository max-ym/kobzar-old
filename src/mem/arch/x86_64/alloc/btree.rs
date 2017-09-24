use super::p2m::Page2m;
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

impl BTreeLeaf {

    /// Get given page status structure pointer if it is stored in this
    /// leaf.
    pub fn get_page(&self, p: &Page2m) -> Option<*mut Page2mStatus> {
        let index = p.addr() / (1024 * 2048) % 4;
        let ptr = self.arr[index as usize];

        if ptr as usize == 0 {
            None
        } else {
            unsafe {
                if (*ptr).page_address() == p.addr() {
                    Some(ptr)
                } else {
                    None
                }
            }
        }
    }
}

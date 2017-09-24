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

    /// Create empty leaf.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get given page status structure pointer if it is stored in this
    /// leaf.
    pub fn get_page(&self, p: &Page2m) -> Option<*mut Page2mStatus> {
        let index = Self::page_to_index(p);
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

    /// Index of array entry for this page.
    fn page_to_index(p: &Page2m) -> usize {
        (p.addr() / (1024 * 2048) % 4) as _
    }

    /// Link given page status to this leaf.
    pub fn link_status(&mut self, p: &Page2mStatus) {
        let index = Self::page_to_index(p.page());

        self.arr[index] = p as *const Page2mStatus as *mut _;
    }

    /// Unlink page status in array at position 0, if any.
    pub fn unlink0(&mut self) {
        self.arr[0] = ::core::ptr::null_mut();
    }

    /// Unlink page status in array at position 1, if any.
    pub fn unlink1(&mut self) {
        self.arr[1] = ::core::ptr::null_mut();
    }

    /// Unlink page status in array at position 2, if any.
    pub fn unlink2(&mut self) {
        self.arr[2] = ::core::ptr::null_mut();
    }

    /// Unlink page status in array at position 3, if any.
    pub fn unlink3(&mut self) {
        self.arr[3] = ::core::ptr::null_mut();
    }

}

impl Default for BTreeLeaf {

    fn default() -> Self {
        BTreeLeaf {
            arr: [0 as *const Page2mStatus as *mut _; 4],
        }
    }
}

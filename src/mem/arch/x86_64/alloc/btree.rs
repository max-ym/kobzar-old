use super::p2m::Page2m;
use super::p2m::Page2mStatus;

/// B-Tree of 2MiB pages.
pub struct PageBTree {
}

/// Node of B-tree of 2MiB pages.
pub struct BTreeNode {
    data    : BTreeLeaf,
    below   : *mut BTreeNode,
    above   : *mut BTreeNode,
}

/// Leaf of B-tree. It was calculated that optimal leaf element count is 4.
/// So each leaf stores 4 pointers to page status structures in the heap.
#[repr(packed)]
pub struct BTreeLeaf {
    arr     : [*mut Page2mStatus; 4],
    base    : u64,
}

impl BTreeLeaf {

    /// Create leaf with given page status structure pointer.
    pub fn new(ps: &Page2mStatus) -> Self {
        let base = Self::page_to_base(ps.page());

        BTreeLeaf {
            arr     : [0 as *const Page2mStatus as *mut _; 4],
            base    : base,
        }
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

    /// Base address of the leaf that stores given page.
    fn page_to_base(p: &Page2m) -> u64 {
        let i = p.addr() / 0x800000;
        i * 0x800000
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

    /// Whether given page is above the range of pages in the leaf.
    pub fn is_above(&self, p: &Page2m) -> bool {
        self.base + 0x200000 * 4 <= p.addr()
    }

    /// Whether given page is below the range of pages in the leaf.
    pub fn is_below(&self, p: &Page2m) -> bool {
        self.base > p.addr()
    }

    /// Whether given page is in this leaf's range.
    pub fn is_in_range(&self, p: &Page2m) -> bool {
        !self.is_above(p) && !self.is_below(p)
    }
}

impl BTreeNode {

    /// Create new BTreeNode that stores pointer to given page status.
    pub fn new(ps: &Page2mStatus) -> Self {
        use core::ptr::null_mut;

        BTreeNode {
            above   : null_mut(),
            below   : null_mut(),
            data    : BTreeLeaf::new(ps),
        }
    }

    /// Get page from this node immediately.
    pub fn get_page_from_node(&self, p: &Page2m) -> Option<*mut Page2mStatus> {
        self.data.get_page(p)
    }
}

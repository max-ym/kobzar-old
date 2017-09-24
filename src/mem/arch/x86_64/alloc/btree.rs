use super::p2m::Page2m;
use super::p2m::Page2mStatus;

/// B-Tree of 2MiB pages.
pub struct BTree {
    root    : *mut BTreeNode,
}

/// Node of B-tree of 2MiB pages.
pub struct BTreeNode {
    data    : BTreeLeaf,
    below   : *mut BTreeNode,
    above   : *mut BTreeNode,
}

const BTREE_KEY_COUNT: u64 = 4;

/// Leaf of B-tree.
#[repr(packed)]
pub struct BTreeLeaf {
    arr     : [*mut Page2mStatus; BTREE_KEY_COUNT as _],
    base    : u64,
}

impl BTree {

    /// Create new empty B-tree.
    pub fn new() -> Self {
        BTree {
            root    : ::core::ptr::null_mut(),
        }
    }
}

impl BTreeLeaf {

    /// Create leaf with given page status structure pointer.
    pub fn new(ps: &Page2mStatus) -> Self {
        let base = Self::page_to_base(ps.page());

        BTreeLeaf {
            arr     : [0 as *const Page2mStatus as *mut _;
                            BTREE_KEY_COUNT as _],
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
        (p.addr() / (1024 * 2048) % BTREE_KEY_COUNT) as _
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

    /// Unlink page status in array at given position, if any.
    ///
    /// # Safety
    /// Intex bounds are not checked before array access. Ensure
    /// index has valid value.
    pub unsafe fn unlink_unchecked(&mut self, index: usize) {
        self.arr[index] = ::core::ptr::null_mut();
    }

    /// Whether given page is above the range of pages in the leaf.
    pub fn is_above(&self, p: &Page2m) -> bool {
        self.base + 0x200000 * BTREE_KEY_COUNT <= p.addr()
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

    /// Get page status structure poiunter from this node immediately.
    pub fn get_page_from_node(&self, p: &Page2m) -> Option<*mut Page2mStatus> {
        self.data.get_page(p)
    }

    /// Get page status structure pointer from this node or it's children.
    pub fn get_page(&self, p: &Page2m) -> Option<*mut Page2mStatus> {
        if self.data.is_above(p) {
            if self.above as usize == 0 {
                None
            } else {
                unsafe { (*self.above).get_page(p) }
            }
        } else if self.data.is_below(p) {
            if self.below as usize == 0 {
                None
            } else {
                unsafe { (*self.below).get_page(p) }
            }
        } else {
            self.get_page_from_node(p)
        }
    }
}

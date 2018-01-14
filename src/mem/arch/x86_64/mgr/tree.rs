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

impl DivData {

    /// Count allocations of page by the index.
    ///
    /// # Safety
    /// Does not check whether index is within array size limits.
    pub unsafe fn allocs_of_page(&self, index: usize) -> ProcessCount {
        self.counters[index]
    }

    /// Whether all pages are not allocated. If at least one is allocated
    /// then false is returned. If all are free then true is returned.
    /// If all pages are free they can be marged to a single 2 MiB page.
    pub fn is_free(&self) -> bool {
        self.map.is_all_zeros()
    }

    /// Whether all pages are allocated. In this case no more new pages could
    /// be allocated. Pages still can be shared between processes and it is
    /// allowed to allocate for sharing. The meaning is to check whether
    /// it is possible to allocate fresh page that is still not used anywhere.
    pub fn is_full(&self) -> bool {
        self.map.is_all_ones()
    }

    /// Allocate unused page. Page's index will be returned.
    pub fn alloc(&mut self) -> Option<usize> {
        let free_index = self.map.first_zero();

        if free_index.is_some() {
            let free_index = free_index.unwrap();

            // Mark page as used in the map.
            self.map.set_one(free_index);

            // Update page allocation counter.
            self.counters[free_index] = 1;

            Some(free_index)
        } else {
            None
        }
    }

    /// Mark page by index as once more shared.
    ///
    /// # Safety
    /// Does not check whether index is valid and whether page was really
    /// allocated.
    pub unsafe fn share(&mut self, index: usize) {
        self.counters[index] += 1;
    }

    /// Decrease share counter. Page is now not used by someone.
    /// Updates bitmap when page share counter reaches zero which means
    /// complete page deallocation and that it's free for use again.
    ///
    /// # Safety
    /// Does not check whether index is valid and whether page was really
    /// allocated.
    pub unsafe fn unshare(&mut self, index: usize) {
        self.counters[index] -= 1;
        if self.counters[index] == 0 {
            self.map.set_zero(index);
        }
    }
}

impl Default for DivData {

    fn default() -> Self {
        DivData {
            map         : Bitmap512::new_zeros(),
            counters    : [0; 512],
        }
    }
}

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

    /// Map that indicates the availability of pages for allocation.
    /// Map increases search speed on x86_64 processors because
    /// it uses a single instruction search for each 64-bits.
    /// So whole map check will be finished much faster than checking
    /// each individual counter from the array.
    map     : Bitmap512,

    /// Array of counters for each page. Counter of page allocation.
    counters: [ProcessCount; 512],
}

/// Error that occurs when 2 MiB page gets divided and fails.
enum PageDivisionError {

    /// Page already is divided and so this operation cannot be performed.
    AlreadyDivided,

    /// Page is allocated as one and so cannot be used for division until
    /// everyone who uses the page releases it.
    Allocated,
}

/// Error that occurs when 2 MiB page gets merged and fails.
enum PageMergeError {

    /// Cannot merge the page that is not divided in first place.
    /// Maybe you're trying to merge the page twice mistakenly?
    /// Divide the page and this error will not occur.
    NotDivided,

    /// Some page parts are still in use among the system. Cannot merge the
    /// page having it's slices used. When all 4 KiB parts get free,
    /// the merge will not cause this error.
    Used,
}

impl ArrayEntry {

    /// Whether the 2 MiB page that is covered by the entry was divided
    /// into 512 4 KiB pages or remains as one 2 MiB.
    pub fn is_divided(&self) -> bool {
        self.is_divided
    }

    /// How many times the 2 MiB page was shared. None returns when page
    /// was divided and cannot be possibly shared as 2 MiB.
    pub fn share_count(&self) -> Option<ProcessCount> {
        if self.is_divided() {
            None
        } else {
            unsafe { Some(self.data.alloc_count) }
        }
    }

    /// Information about page division. if covered page was not yet divided
    /// then None will be returned.
    pub fn div_data(&self) -> Option<&DivData> {
        if self.is_divided() {
            unsafe { Some(&*self.data.div_data) }
        } else {
            None
        }
    }

    /// Information about page division. if covered page was not yet divided
    /// then None will be returned.
    pub fn div_data_mut(&mut self) -> Option<&mut DivData> {
        if self.is_divided() {
            unsafe { Some(&mut *self.data.div_data) }
        } else {
            None
        }
    }

    /// Divide given 2 MiB page into 512 4 KiB pages. For Err details see
    /// enum.
    ///
    /// # Safety
    /// Caller must provide proper memory for storing DivData. Failing to
    /// do so may cause data corruption and errors.
    pub unsafe fn divide(&mut self, div_data_storage: *mut DivData)
            -> Result<(), PageDivisionError> {
        use self::PageDivisionError::*;

        if self.is_divided() {
            return Err(AlreadyDivided);
        }
        if self.share_count().unwrap() > 0 {
            return Err(Allocated);
        }

        self.is_divided = true;
        unsafe { self.data.div_data = div_data_storage; }
        Ok(())
    }

    /// Merge divided 2 MiB page back into one. For Err details see
    /// enum. On success returns the DivData memory pointer that was used
    /// to store the division data and is no longer needed. Memory allocator
    /// may free that memory or system may use it in other way now.
    pub fn merge(&mut self) -> Result<*mut DivData, PageMergeError> {
        use self::PageMergeError::*;

        if !self.is_divided() {
            return Err(NotDivided);
        }
        if !self.div_data().unwrap().is_free() {
            return Err(Used);
        }

        self.is_divided = false;
        unsafe { Ok(self.data.div_data) }
    }
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
    /// Does not check whether index is valid and whether page is really
    /// allocated. For new page allocation use 'alloc' fn.
    pub unsafe fn share(&mut self, index: usize) {
        self.counters[index] += 1;
    }

    /// Decrease share counter. Shows that page is now not used by someone.
    /// Updates bitmap when page share counter reaches zero which means
    /// complete page deallocation and that it's free for use again.
    ///
    /// # Safety
    /// Does not check whether index is valid and whether page is really
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

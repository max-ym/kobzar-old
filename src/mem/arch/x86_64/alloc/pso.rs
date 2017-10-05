use super::Page2m;
use super::Page2mStatus as PageStatus;

/// Page ranges.
struct Range {
    pub low     : u64,
    pub hi      : u64,
}

pub struct PSArray {
    range   : Range,
    arr     : *mut PageStatus,
}

/// PSA array.
pub struct PSAArray {
    length  : u32,
    arr     : *mut PSArray,
}

impl Range {

    /// Create new range.
    pub fn new(top: u64, bottom: u64) -> Self {
        Range {
            hi  : top,
            low : bottom,
        }
    }

    /// How many entries this range contains.
    pub fn length(&self) -> u64 {
        (self.hi - self.low) / 0x200000
    }

    /// Get index of a page status entry for this page address.
    ///
    /// # Safety
    /// It is expected that given absolute address of a page is
    /// within this range.
    pub unsafe fn abs_to_index(&self, absolute: u64) -> u64 {
        (absolute - self.low) / 0x200000
    }

    /// Whether this page is within the range.
    pub fn is_page_included(&self, page: Page2m) -> bool {
        let addr = page.addr();
        self.low >= addr && self.hi <= addr
    }
}

impl ::core::ops::Index<u64> for PSArray {

    type Output = PageStatus;

    fn index(&self, index: u64) -> &Self::Output {
        unsafe { &*self.arr.offset(index as _) }
    }
}

impl ::core::ops::IndexMut<u64> for PSArray {

    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        unsafe { &mut *self.arr.offset(index as _) }
    }
}

impl PSArray {

    /// Get page that page status at given position is saving status for.
    pub fn page_at_index(&self, index: u64) -> Page2m {
        let addr = self.range.low + index * 0x200000;
        Page2m::new(addr)
    }

    /// Whether this page is within the range.
    pub fn is_page_included(&self, page: Page2m) -> bool {
        self.range.is_page_included(page)
    }
}

impl ::core::ops::Index<u64> for PSAArray {

    type Output = PSArray;

    fn index(&self, index: u64) -> &Self::Output {
        unsafe { &*self.arr.offset(index as _) }
    }
}

impl ::core::ops::IndexMut<u64> for PSAArray {

    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        unsafe { &mut *self.arr.offset(index as _) }
    }
}

impl PSAArray {

    /// Find array that contains this page.
    ///
    /// # Safety
    /// This method outputs reference even when no array contains this page.
    /// The reference will be null in this case and must not be used.
    pub unsafe fn containing_array_unsafe(&self, page: Page2m) -> &PSArray {
        for i in 0..self.length {
            let i = i as u64;
            if self[i].is_page_included(page) {
                return &self[i];
            }
        }

        &*::core::ptr::null()
    }
}

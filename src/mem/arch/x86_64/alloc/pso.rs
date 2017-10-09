use super::Page2m;
use super::Range2m;

/// Page Status. Holds the state of individual 2MiB or 4KiB pages.
/// Stores whether it is allocated or free.
#[derive(Default)]
pub struct PageStatus {

    /// Counter of how many table entries contain this page.
    /// When counter is zero, this page is free.
    used    : u32,
}

/// Page Status array.
pub struct PsArray {
    range   : Range2m,
    arr     : *mut PageStatus,
}

/// PSA array. Contains all Page Status arrays of the system.
pub struct PsaArray {
    length  : u32,
    arr     : *mut PsArray,
}

impl PageStatus {

    /// How many tables use this page.
    pub fn use_count(&self) -> u32 {
        self.used
    }

    /// Notify that one more table uses this page now.
    /// Increments user counter.
    ///
    /// Returns new user counter value.
    pub fn inc_user(&mut self) -> u32 {
        self.used += 1;
        self.used
    }

    /// Notify that one table released this page now.
    /// Decrements user counter.
    ///
    /// Returns new user counter value.
    pub fn dec_user(&mut self) -> u32 {
        self.used -= 1;
        self.used
    }

    /// Set given user counter value.
    pub fn set_user(&mut self, val: u32) {
        self.used = val;
    }

    /// Whether this page is allocated for some page table.
    ///
    /// Opposite to fn `is_free`.
    pub fn is_used(&self) -> bool {
        self.used > 0
    }

    /// Whether this page is free to be allocated.
    ///
    /// Opposite to fn `is_used`.
    pub fn is_free(&self) -> bool {
        self.used == 0
    }
}

impl ::core::ops::Index<u64> for PsArray {

    type Output = PageStatus;

    fn index(&self, index: u64) -> &Self::Output {
        unsafe { &*self.arr.offset(index as _) }
    }
}

impl ::core::ops::IndexMut<u64> for PsArray {

    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        unsafe { &mut *self.arr.offset(index as _) }
    }
}

impl ::core::ops::Index<Page2m> for PsArray {

    type Output = PageStatus;

    fn index(&self, page: Page2m) -> &Self::Output {
        unsafe { self.page_status_for(page) }
    }
}

impl ::core::ops::IndexMut<Page2m> for PsArray {

    fn index_mut(&mut self, page: Page2m) -> &mut Self::Output {
        unsafe { self.page_status_mut_for(page) }
    }
}

impl PsArray {

    /// Get page that page status at given position is saving status for.
    pub fn page_at_index(&self, index: u64) -> Page2m {
        let addr = self.range.bottom() + index * 0x200000;
        Page2m::new(addr)
    }

    /// Whether this page is within the range.
    pub fn contains(&self, page: Page2m) -> bool {
        self.range.contains(page)
    }

    pub unsafe fn page_status_for(&self, page: Page2m) -> &PageStatus {
        &*self.arr.offset(self.page_to_index(page) as _)
    }

    pub unsafe fn page_status_mut_for(&mut self, page: Page2m)
            -> &mut PageStatus {
        &mut *self.arr.offset(self.page_to_index(page) as _)
    }

    unsafe fn page_to_index(&self, page: Page2m) -> u64 {
        self.range.abs_to_index(page.addr())
    }
}

impl ::core::ops::Index<u64> for PsaArray {

    type Output = PsArray;

    fn index(&self, index: u64) -> &Self::Output {
        unsafe { &*self.arr.offset(index as _) }
    }
}

impl ::core::ops::IndexMut<u64> for PsaArray {

    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        unsafe { &mut *self.arr.offset(index as _) }
    }
}

impl ::core::ops::Index<Page2m> for PsaArray {

    type Output = PsArray;

    fn index(&self, page: Page2m) -> &Self::Output {
        unsafe { self.array_with_page_unsafe(page) }
    }
}

impl ::core::ops::IndexMut<Page2m> for PsaArray {

    fn index_mut(&mut self, page: Page2m) -> &mut Self::Output {
        unsafe { self.array_with_page_mut_unsafe(page) }
    }
}

impl PsaArray {

    /// Find array that contains this page.
    ///
    /// # Safety
    /// This method outputs reference even when no array contains this page.
    /// The reference will be null in this case and must not be used.
    pub unsafe fn array_with_page_unsafe(&self, page: Page2m) -> &PsArray {
        for i in 0..self.length {
            let i = i as u64;
            if self[i].contains(page) {
                return &self[i];
            }
        }

        &*::core::ptr::null()
    }

    pub unsafe fn array_with_page_mut_unsafe(&mut self, page: Page2m)
            -> &mut PsArray {
        let notmut = self.array_with_page_unsafe(page);

        // Convert to mutable reference
        &mut *(notmut as *const PsArray as *mut PsArray)
    }

    /// Page Status entry for given page.
    ///
    /// # Safety
    /// Does not check whether this page has status entry or not. Entry
    /// presence must be guaranteed by caller.
    pub unsafe fn page_status_for(&self, page: Page2m) -> &PageStatus {
        let psa = self.array_with_page_unsafe(page);
        &psa[page]
    }

    /// Page Status entry for given page.
    ///
    /// # Safety
    /// Does not check whether this page has status entry or not. Entry
    /// presence must be guaranteed by caller.
    pub unsafe fn page_status_mut_for(&mut self, page: Page2m)
            -> &mut PageStatus {
        let psa = self.array_with_page_mut_unsafe(page);
        &mut psa[page]
    }
}

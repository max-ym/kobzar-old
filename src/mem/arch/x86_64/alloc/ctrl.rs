use super::Page4k;
use super::Page2m;
use super::Stack2m;
use super::PageStatus;
use super::PsaArray;

type AlResult<T> = ::core::result::Result<T, AllocError>;
type ReResult<T> = ::core::result::Result<T, ReleaseError>;

/// Pages allocator.
pub struct Alloc {

    /// Stack that contains all 2MiB pages that are free for allocation.
    stk2    : Stack2m,

    /// Array that contains 2MiB page ranges that are controlled by memory
    /// controller. Each range itself is a fixed size array with page status
    /// objects of individual pages.
    psa     : PsaArray,

    // TODO
}

/// Handle that allows to control the 2MiB page status and get page address.
pub struct Page2mHandle {
    page    : Page2m,
    stat    : *mut PageStatus,
}

/// Handle that allows to control the 4KiB page status and get page address.
pub struct Page4kHandle {
    page    : Page4k,
    stat    : *mut PageStatus,
}

/// Errors that allocator can return when some action cannot be
/// performed for any reason.
pub enum AllocError {

    /// No more pages are available for allocation.
    NoMorePages
}

pub enum ReleaseError {

    /// Page usage counter is not zero. Maybe the page is still used by some
    /// tables and thus cannot be released.
    UsageCounterNonzero,
}

macro_rules! impl_page_handle {
    ($p:ty) => (
        pub fn page(&self) -> $p {
            self.page
        }

        pub fn status(&self) -> &PageStatus {
            unsafe { &*self.stat }
        }

        pub fn status_mut(&mut self) -> &mut PageStatus {
            unsafe { &mut *self.stat }
        }
    )
}

impl Page2mHandle {
    impl_page_handle!(Page2m);
}

impl Page4kHandle {
    impl_page_handle!(Page4k);
}

impl Alloc {

    pub fn alloc4k(&mut self) -> AlResult<Page4kHandle> {
        unimplemented!()
    }

    pub fn alloc2m(&mut self) -> AlResult<Page2mHandle> {
        unimplemented!()
    }

    pub unsafe fn release4k(&mut self, page: Page4kHandle) -> ReResult<()> {
        unimplemented!()
    }

    pub unsafe fn release2m(&mut self, page: Page2mHandle) -> ReResult<()> {
        unimplemented!()
    }

    /// Amount of free 2MiB pages.
    pub fn free2m_pages(&self) -> usize {
        unimplemented!()
    }

    /// Amount of free memory in bytes that are covered by 2MiB pages.
    pub fn free2m_bytes(&self) -> usize {
        let page2m_size = 2048 * 1024;
        self.free2m_pages() * page2m_size
    }

    /// Amount of free 4KiB pages.
    pub fn free4k_pages(&self) -> usize {
        unimplemented!()
    }

    /// Amount of free memory in bytes that are covered by 4KiB pages.
    pub fn free4k_bytes(&self) -> usize {
        let page4k_size = 4096;
        self.free4k_pages() * page4k_size
    }

    /// Amount of free memory.
    pub fn free_memory_size(&self) -> usize {
        self.free2m_bytes() + self.free4k_bytes()
    }
}

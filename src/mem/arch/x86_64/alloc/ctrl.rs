use super::Page4k;
use super::Page2m;
use super::Stack2m;
use super::PageStatus;

type Result<T> = ::core::result::Result<T, AllocError>;

/// Pages allocator.
pub struct Alloc {

    /// Stack that contains all 2MiB pages that are free for allocation.
    stk2    : Stack2m,
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

    pub fn alloc4k(&mut self) -> Result<Page4kHandle> {
        unimplemented!()
    }

    pub fn alloc2m(&mut self) -> Result<Page2mHandle> {
        unimplemented!()
    }

    pub unsafe fn release4k(&mut self, page: Page4k) -> Result<()> {
        unimplemented!()
    }

    pub unsafe fn release2m(&mut self, page: Page2m) -> Result<()> {
        unimplemented!()
    }

    /// Amount of free 2MiB pages.
    pub fn free2m_size(&self) -> usize {
        unimplemented!()
    }

    /// Amount of free memory in bytes that are covered by 2MiB pages.
    pub fn free2m_bytes(&self) -> usize {
        self.free2m_size() * 2048 * 1024
    }

    /// Amount of free 4KiB pages.
    pub fn free4k_size(&self) -> usize {
        unimplemented!()
    }

    /// Amount of free memory in bytes that are covered by 4KiB pages.
    pub fn free4k_bytes(&self) -> usize {
        self.free4k_size() * 4096
    }

    /// Amount of free memory.
    pub fn free_memory_size(&self) -> usize {
        self.free2m_bytes() + self.free4k_bytes()
    }
}

use super::Page4k;
use super::Page2m;

type Result<T> = ::core::result::Result<T, AllocError>;

/// Pages allocator.
pub struct Alloc {
    // TODO
}

/// Errors that allocator can return when some action cannot be
/// performed for any reason.
pub enum AllocError {
}

impl Alloc {

    pub fn alloc4k(&mut self) -> Result<Page4k> {
        unimplemented!()
    }

    pub fn alloc2m(&mut self) -> Result<Page2m> {
        unimplemented!()
    }

    pub fn release4k(&mut self, page: Page4k) -> Result<()> {
        unimplemented!()
    }

    pub fn release2m(&mut self, page: Page2m) -> Result<()> {
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

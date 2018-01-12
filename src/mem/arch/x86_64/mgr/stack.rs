use super::Page2m;
use collections::FreeStack;

/// Stack of 2MiB pages. Wrapper for the stack structure from 'collections'.
pub struct StackP2 {
    base    : FreeStack<Page2m>,
}

impl StackP2 {

    /// Create new stack structure that has it's top set to NULL address.
    pub fn new() -> Self {
        StackP2 {
            base    : FreeStack::new(::core::ptr::null_mut()),
        }
    }

    /// Pop next page from the stack.
    pub fn pop_page(&mut self) -> Option<Page2m> {
        self.base.pop()
    }

    /// Push page that is now free for use.
    pub fn push_page(&mut self, page: Page2m) {
        self.base.push(page)
    }

    /// Count of free pages.
    pub fn count(&self) -> usize {
        self.base.count()
    }

    /// Access the base stack structure.
    pub fn base(&self) -> &FreeStack<Page2m> {
        &self.base
    }

    /// Access the base stack structure.
    ///
    /// # Safety
    /// Mutating the stack is discouraged
    /// because some important changes will not be tracked by the wrapper
    /// which may lead to it's corruption.
    pub unsafe fn base_mut(&mut self) -> &mut FreeStack<Page2m> {
        &mut self.base
    }
}

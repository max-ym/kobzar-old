//! Process List Controller module.

use super::ProcessHandle;

/// Error of process allocation.
pub enum ProcessAllocErr {
}

/// Process Handle Set. This set stores all process handles. Different
/// scheduler process lists point to process handles in this set.
pub trait HandleSet {

    /// Process Handle type that is stored in this Process Handle Set.
    type P : ProcessHandle;

    /// Get process with given ID.
    fn process_by_id(&self, id: u32) -> Option<&Self::P>;

    /// Get process with given ID.
    fn process_by_id_mut(&mut self, id: u32) -> Option<&mut Self::P> {
        match self.process_by_id(id) {
            Some(t) => Some(unsafe { &mut *(t as *const Self::P as *mut _) }),
            None    => None
        }
    }

    /// Remove process with given ID.
    ///
    /// # Safery
    /// External controller must ensure no more pointers for this
    /// process exist. Otherwise, remain pointers will become dangling.
    unsafe fn remove_id(&mut self, id: u32);

    /// Remove process by given handle reference.
    ///
    /// # Safery
    /// External controller must ensure no more pointers for this
    /// process exist. Otherwise, remain pointers will become dangling.
    unsafe fn remove(&mut self, p: &Self::P) {
        self.remove_id(p.id())
    }

    /// Create new process entry in this set.
    fn new_process(&mut self) -> Result<Self::P, ProcessAllocErr>;
}

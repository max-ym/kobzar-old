//! Process List Controller module.

/// Process Handle Set. This set stores all process handles. Different
/// scheduler process lists point to process handles in this set.
pub trait HandleSet {

    /// Process Handle type that is stored in this Process Handle Set.
    type P : super::ProcessHandle;

    /// Get process with given ID.
    fn process_by_id(&self, id: u32) -> Option<Self::P>;

    /// Remove process with given ID.
    ///
    /// # Safery
    /// External controller must ensure no more pointers for this
    /// process exist. Otherwise, remain pointers will become dangling.
    unsafe fn remove_id(&mut self, id: u32);

    /// Create new process entry in this set.
    ///
    /// TODO errors that may occur.
    fn new_process(&mut self) -> Result<Self::P, ()>;
}

/// Process List Node.
pub struct ProcessListNode<PH : super::ProcessHandle> {

    /// Next node of the list.
    next: *mut ProcessListNode<PH>,

    /// Pointer to Process Handle.
    ptr: *mut PH,
}

/// Process List.
pub struct ProcessList<PH : super::ProcessHandle> {

    /// The top of the list. May be NULL if list is empty.
    top: *mut ProcessListNode<PH>
}

/// Scheduler Process List Controller.
/// Process List Controller store all processes that are registered in
/// scheduler with all metadata that is required to identify
/// the process, it's state, set up process environment, save process data on
/// context switches etc.
pub struct ProcessListController<PH : super::ProcessHandle> {
    paused_tasks    : ProcessList<PH>,
    queued_tasks    : ProcessList<PH>,
    running_tasks   : ProcessList<PH>,

    paused_procs    : ProcessList<PH>,
    vacant_procs    : ProcessList<PH>,
    running_procs   : ProcessList<PH>,
}

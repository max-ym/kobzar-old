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

    /// Paused Tasks that were paused by something external and will continue
    /// executing when someone unpause them back.
    paused_tasks    : ProcessList<PH>,

    /// Tasks that wait their turn in execution queue.
    queued_tasks    : ProcessList<PH>,

    /// Tasks that got their processor time and are currently running.
    running_tasks   : ProcessList<PH>,

    /// Processes that were paused by something external. They will
    /// continue executing when someone gets them unpaused again.
    paused_procs    : ProcessList<PH>,

    /// Processes that wait when scheduler gives them processor time.
    vacant_procs    : ProcessList<PH>,

    /// Processes that got their processor time and are currently running.
    running_procs   : ProcessList<PH>,
}

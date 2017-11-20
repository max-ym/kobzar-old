//! Process List Controller module.

use super::ProcessHandle;

/// Error of process allocation.
pub enum ProcessAllocErr {
}

/// Process handle set. This set stores all process handles. Different
/// scheduler process lists point to process handles in this set.
pub trait ProcessSet {

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

/// Queue of processes. Processes in the queue are automatically
/// sorted in the right order
/// they are expected to be executed according to their properties like
/// priority etc.
pub trait ProcessQueue {

    /// Process Handle type that is queued.
    type P : ProcessHandle;

    /// Peek current value in queue if any.
    fn peek(&self) -> Option<&Self::P>;

    /// Peek current value in queue if any.
    fn peek_mut(&mut self) -> Option<&mut Self::P> {
        match self.peek() {
            Some(t) => Some(unsafe { &mut *(t as *const Self::P as *mut _)}),
            None    => None
        }
    }

    /// Take current value from the queue and move to the next one.
    /// This value is moved out and will be not in queue anymore.
    fn next(&mut self) -> Option<&mut Self::P>;

    /// Append given value to the queue.
    fn append(&mut self, p: &mut Self::P);
}

/// Heap of queues, lists and sets of processes used in scheduler architecture.
pub trait SchedProcesses {

    /// Paused tasks handle set.
    type PT : ProcessSet;

    /// Queue of tasks.
    type TQ : ProcessQueue;

    /// Paused processes.
    type PP : ProcessSet;

    /// Vacant processes that wait their processor time.
    type VP : ProcessQueue;

    /// Paused tasks.
    fn paused_tasks(&self) -> &Self::PT;

    /// Paused tasks.
    fn paused_tasks_mut(&mut self) -> &mut Self::PT {
        unsafe { &mut *(self.paused_tasks() as *const Self::PT as *mut _) }
    }

    /// Tasks in queue that are waiting for processor time.
    fn task_queue(&self) -> &Self::TQ;

    fn task_queue_mut(&mut self) -> &mut Self::TQ {
        unsafe { &mut *(self.task_queue() as *const Self::TQ as *mut _) }
    }

    /// Processes that were paused by external thing. They will continue
    /// executing when they get unpaused by someone.
    fn paused_procs(&self) -> &Self::PP;

    fn paused_procs_mut(&mut self) -> &mut Self::PP {
        unsafe { &mut *(self.paused_procs() as *const Self::PP as *mut _) }
    }

    /// Processes that wait their processor time.
    fn vacant_procs(&self) -> &Self::VP;

    fn vacant_procs_mut(&mut self) -> &mut Self::VP {
        unsafe { &mut *(self.vacant_procs() as *const Self::VP as *mut _) }
    }
}

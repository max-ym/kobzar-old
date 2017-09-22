
/// Process is a runnable instance of some service with allocated
/// metadata and working environment.
trait Process {
}

/// All process states.
enum ProcessState {

    /// Process is currently running.
    Running,

    /// Process has ended execution or was terminated.
    End,

    /// Process was paused by something.
    Pause,

    /// Scheduler removed processor time from this process.
    SchedulerPause,

    /// Process is waiting for external signal to wake him up.
    Wait,

    /// Process is waiting for external signal to wake him up for limited time.
    /// If time goes out, timer will wake this process up.
    TimedWait,
}

/// Handle of process for specific sheduler implementation.
trait ProcessHandle : Sized {

    /// Process that trait handles.
    type P : Process;

    /// Process current state.
    fn state(&self) -> ProcessState;

    /// Store this process in paused list if possible.
    /// If process is terminated, finished etc. it cannot be stored in the
    /// list.
    fn save_as_paused(&mut self) -> Result<(),()>;

    /// Store this process in vacant process list if possible.
    /// If process is terminated, finished etc. it cannot be stored in the
    /// list.
    fn save_as_vacant(&mut self) -> Result<(),()>;

    /// Remove the process from the scheduler.
    /// Method consumes the process handle.
    /// It can be removed only if process state is End.
    /// Otherwise, the process handle will be returned back in Option.
    fn try_remove(self) -> Option<Self> {
        if self.is_removable() {
            unsafe { self.remove(); }
            None
        } else {
            Some(self)
        }
    }

    /// Remove the process from the scheduler.
    /// Method consumes the process handle.
    ///
    /// # Safety
    /// This method does not check if it is allowed to remove the process.
    /// It can break scheduler or cause undefined behaviour.
    /// Method can be used with explicit check whether removing is safe.
    unsafe fn remove(self);

    /// Check if process can be removed from scheduler.
    /// It can be removed only if process state is End.
    fn is_removable(&self) -> bool {
        match self.state() {
            ProcessState::End => true,
            _                 => false,
        }
    }
}

/// The core mechanisms of scheduler which are not visible outside this
/// module.
trait Scheduler {

    /// Process handle of this scheduler.
    type P : ProcessHandle;

    /// Get next process to run and remove it from vacant process list.
    fn next_vacant_process(&mut self) -> Self::P;
}


/// Process is a runnable instance of some service with allocated
/// metadata and working environment.
pub trait Process {
}

/// All process states.
pub enum ProcessState {

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
trait ProcessHandle {

    /// Process that trait handles.
    type P : Process;

    /// Process current state.
    fn state(&self) -> ProcessState;

    /// Store this process in paused list if possible.
    /// If process is terminated, finished etc. it cannot be stored in the
    /// list.
    fn save_as_paused(&mut self) -> Option<()>;

    /// Store this process in vacant process list if possible.
    /// If process is terminated, finished etc. it cannot be stored in the
    /// list.
    fn save_as_vacant(&mut self) -> Option<()>;

    /// Remove the process from the scheduler.
    /// Method consumes the process handle.
    /// It can be removed only if process state is End.
    /// Otherwise, the process handle will be returned back in Option.
    fn remove(self) -> Option<Self>;

    /// Check if process can be stored either in paused or vacant process
    /// list.
    fn is_saveable(&self) -> bool {
        use self::ProcessState::*;
        match self.state() {
            Running         => true,
            Pause           => true,
            SchedulerPause  => true,

            End             => false,
            Wait            => false,
            TimedWait       => false,
        }
    }

    /// Check if process can be removed from scheduler.
    /// It can be removed only if process state is End.
    fn is_removable(&self) -> bool {
        self.state() == ProcessState::End
    }
}

/// The core mechanisms of scheduler which are not visible outside this
/// module.
trait SchedulerCore : Scheduler {

    /// Process handle of this scheduler.
    type P : ProcessHandle;

    /// Get next process to run and remove it from vacant process list.
    fn next_vacant_process(&mut self) -> Self::P;
}

/// External visible part of scheduler.
pub trait Scheduler {
}

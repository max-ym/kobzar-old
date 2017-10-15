mod list;
use self::list::*;

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
pub trait ProcessHandle : Sized {

    /// Process current state.
    fn state(&self) -> ProcessState;
}

/// The core mechanisms of scheduler which are not visible outside this
/// module.
trait Core {

    /// The Process Handle of the Scheduler implementation.
    type PH : ProcessHandle;

    /// Pop next process that is waiting in the queue for processor time.
    fn pop_next_proc(&mut self) -> Self::PH;
}


/// Process is a runnable instance of some service with allocated
/// metadata and working environment.
pub trait Process {

    /// Process current state.
    pub state(&self) -> ProcessState;
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

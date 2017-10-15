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

/// The CPU unit that is running single thread. Used to assign to it
/// tasks and processes.
trait ProcessorThread {

    /// The Process Handle of this Processor Thread implementation.
    type PH : ProcessHandle;

    /// Assign new process to execute on this unit. Old process is returned
    /// back.
    fn assign_process(&mut self, pr: &Self::PH) -> &Self::PH;

    /// Current process assigned to this unit.
    fn current_process(&self) -> &Self::PH;
}

/// The core mechanisms of scheduler which are not visible outside this
/// module.
trait Core {

    /// The Process Handle of the Scheduler implementation.
    type PH : ProcessHandle;

    /// The Processor Thread of the Scheduler implementation.
    type PT : ProcessorThread;

    /// Pop next process that is waiting in the queue for processor time.
    fn pop_next_proc(&mut self) -> Self::PH;

    /// Pop next task that is waiting in the queue for processor time.
    fn pop_next_task(&mut self) -> Self::PH;

    /// Processor at specified position of processor array.
    fn processor_by_id(&self, id: usize) -> Option<&ProcessorThread>;

    /// Processor at specified position of processor array.
    fn processor_by_id_mut(&mut self, id: usize)
            -> Option<&mut ProcessorThread>;

    /// Count processor units used by this scheduler.
    fn processor_count(&self) -> usize;
}

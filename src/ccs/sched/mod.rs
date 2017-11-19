mod arch;
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

    // Data that must be saved when context switches. This can be registers,
    // stack/code pointer etc.
    type ProcData;

    /// Process current state.
    fn state(&self) -> ProcessState;
}

/// The CPU unit that is running single thread. Used to assign to it
/// tasks and processes.
trait ProcessorUnit {

    /// The Process Handle of this Processor Unit implementation.
    type PH : ProcessHandle;

    /// Assign new process to execute on this unit. Old process is returned
    /// back.
    fn assign_process(&mut self, pr: &Self::PH) -> &Self::PH;

    /// Current process assigned to this unit.
    fn current_process(&self) -> &Self::PH;
}

/// Array of processor units. This array is stored in scheduler.
/// It uses it to know which units it can access and which
/// processes are assigned to any of these units.
trait ProcessorArray<T> where T : ProcessorUnit {

    /// Count processors in this array.
    fn count(&self) -> usize;

    /// Get specific processor unit. If index is out of range None will
    /// be returned.
    fn get(&self, id: usize) -> Option<&T>;

    /// Get specific processor unit. If index is out of range None will
    /// be returned.
    fn get_mut(&mut self, id: usize) -> Option<&mut T>;
}

/// The core mechanisms of scheduler which are not visible outside this
/// module.
trait SchedulerPriv {

    /// The Process Handle of the Scheduler implementation.
    type PH : ProcessHandle;

    /// The Processor Unit of the Scheduler implementation.
    type PT : ProcessorUnit;

    /// Pull out of queue next process that is waiting for processor time.
    fn pull_next_proc(&mut self) -> &mut Self::PH;

    /// Pull out of queue next task that is waiting for processor time.
    fn pull_next_task(&mut self) -> &mut Self::PH;

    /// Array of processors used by this scheduler.
    fn processors(&self) -> &ProcessorArray<Self::PT>;
}

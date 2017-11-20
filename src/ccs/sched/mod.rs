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

/// Handle of the process for specific scheduler implementation.
pub trait ProcessHandle : Sized {

    // Data that must be saved when context switches. This can be registers,
    // stack/code pointer etc.
    type ProcData;

    /// Process current state.
    fn state(&self) -> ProcessState;

    /// Process ID value.
    fn id(&self) -> u32;
}

/// The CPU unit that is running single thread. Used to assign to it
/// tasks and processes.
trait ProcessorUnit {

    /// The Process Handle of this Processor Unit implementation.
    type PH : ProcessHandle;

    /// Assign new process to execute on this unit. Old process is returned
    /// back.
    fn assign_process(&mut self, pr: &Self::PH) -> Option<&Self::PH>;

    /// Current process assigned to this unit.
    fn current_process(&self) -> Option<&Self::PH>;

    /// Halt the processor. Function returns assigned process if any.
    fn halt(&mut self) -> Option<&Self::PH>;
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

    /// Process lists and queues that are used by this scheduler architecture.
    type SP : SchedProcesses;

    /// Array of processors used by this scheduler.
    fn processors(&self) -> &ProcessorArray<Self::PT>;

    /// Array of processors used by this scheduler.
    fn processors_mut(&mut self) -> &mut ProcessorArray<Self::PT> {
        let p = self.processors() as *const ProcessorArray<Self::PT> as *mut _;
        unsafe { &mut *p }
    }

    /// This function meant to be called by system timer to change
    /// current processes that are running.
    fn change_proc_timer_signal(&mut self);

    /// Process lists and queues that are used by this scheduler architecture.
    fn processes(&self) -> &Self::SP;

    /// Process lists and queues that are used by this scheduler architecture.
    fn processes_mut(&mut self) -> &mut Self::SP {
        unsafe { &mut *(self.processes() as *const Self::SP as *mut _) }
    }
}

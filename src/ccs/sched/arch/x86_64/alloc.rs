use mem;
use super::process::ProcessH;

/// Process Handle Allocator.
pub struct PHandleAlloc {

}

/// A single frame of allocator that covers 64 entries
/// process handles.
pub struct PhAllocFrame {
    bitmap  : u64,
    mem     : [*mut PHandleAlloc; 64],
}

impl PHandleAlloc {

    /// Find frame with free cell.
    fn frame_with_free(&self) -> &PhAllocFrame {
        unimplemented!()
    }

    /// Find frame with free cell.
    fn frame_with_free_mut(&mut self) -> &mut PHandleAlloc {
        let ptr = self.frame_with_free() as *const PhAllocFrame as *mut _;
        unsafe { &mut *ptr }
    }

    /// Allocate new process.
    pub fn alloc(&mut self) -> &mut ProcessH {
        unimplemented!()
    }
}

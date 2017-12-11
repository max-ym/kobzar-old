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
    fn frame_with_free(&self) -> Option<&PhAllocFrame> {
        unimplemented!()
    }

    /// Find frame with free cell.
    fn frame_with_free_mut(&mut self) -> Option<&mut PHandleAlloc> {
        let option = self.frame_with_free();
        if option.is_none() { return None; }
        let ptr = option.unwrap();

        let ptr = ptr as *const PhAllocFrame as *mut _;
        Some(unsafe { &mut *ptr })
    }

    /// Allocate new frame.
    fn alloc_new_frame(&mut self) -> &PhAllocFrame {
        unimplemented!();
    }

    /// Allocate new process.
    pub fn alloc(&mut self) -> &mut ProcessH {
        unimplemented!()
    }
}

impl PhAllocFrame {

    /// Allocate entry in this frame. None if frame has all handles allocated.
    pub fn alloc_entry(&self) -> Option<&mut PHandleAlloc> {
        unimplemented!();
    }
}

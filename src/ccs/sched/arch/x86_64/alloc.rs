use mem;
use mem::Address;
use super::process::ProcessH;

/// Process Handle Allocator.
pub struct PHandleAlloc {

}

/// A single frame of allocator that covers 64 entries
/// process handles.
pub struct PhAllocFrame {
    bitmap  : u64,
    mem     : [ProcessH; 64],
}

impl PHandleAlloc {

    /// Find frame with free cell.
    fn frame_with_free(&self) -> Option<&PhAllocFrame> {
        unimplemented!()
    }

    /// Find frame with free cell.
    fn frame_with_free_mut<'a, 'b>(&'a mut self)
            -> Option<&'b mut PhAllocFrame> {
        let option = self.frame_with_free();
        if option.is_none() { return None; }
        let ptr = option.unwrap();

        let ptr = ptr as *const PhAllocFrame as *mut _;
        Some(unsafe { &mut *ptr })
    }

    /// Allocate new frame.
    fn alloc_new_frame(&mut self) -> &mut PhAllocFrame {
        unimplemented!();
    }

    /// Allocate new process.
    pub fn alloc(&mut self) -> &mut ProcessH {
        let option = self.frame_with_free_mut();
        let frame = option.unwrap_or_else(move || {
            self.alloc_new_frame()
        });

        let entry = frame.alloc_entry().unwrap();
        entry
    }

    /// Deallocate existing process.
    pub fn dealloc(&mut self, ph: *mut ProcessH) -> Result<(),()> {
        unimplemented!()
    }
}

impl PhAllocFrame {

    /// Allocate entry in this frame. None if frame has all handles allocated.
    pub fn alloc_entry<'a, 'b>(&'a mut self) -> Option<&'b mut ProcessH> {
        let index = self.free_entry_index();
        if index.is_none() { return None; }
        let index = index.unwrap();

        self.mark_as_allocated(index);

        let r = &mut self.mem[index];
        let ptr = r as *const ProcessH as *mut ProcessH;
        Some(unsafe { &mut *ptr })
    }

    /// Index of free element.
    fn free_entry_index(&self) -> Option<usize> {
        unimplemented!()
    }

    /// Mark frame element by given index as allocated.
    fn mark_as_allocated(&mut self, index: usize) {
        unimplemented!()
    }

    /// Mark frame element by given index as free for allocation.
    fn mark_as_free(&mut self, index: usize) {
        unimplemented!()
    }
}
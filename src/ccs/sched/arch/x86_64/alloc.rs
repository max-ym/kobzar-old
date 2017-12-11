use mem;
use mem::Address;
use super::process::ProcessH;
use collections::Bitmap64;

/// Process Handle Allocator.
pub struct PHandleAlloc {

}

/// A single frame of allocator that covers 64 entries
/// process handles.
pub struct PhAllocFrame {
    bitmap  : Bitmap64,
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

    /// Deallocate frame by given pointer.
    pub fn dealloc_frame(&mut self, fr: *mut PhAllocFrame) {
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
        self.bitmap.first_one()
    }

    /// Mark frame element by given index as allocated.
    fn mark_as_allocated(&mut self, index: usize) {
        self.bitmap.set_zero(index)
    }

    /// Mark frame element by given index as free for allocation.
    fn mark_as_free(&mut self, index: usize) {
        self.bitmap.set_one(index)
    }

    /// Deallocate process.
    pub fn dealloc(&mut self, ph: *mut ProcessH) -> Result<(),()> {
        let index = self.index_of(ph);
        if index.is_none() {
            return Err(());
        }
        let index = index.unwrap();

        self.mark_as_free(index);
        Ok(())
    }

    /// Index of entry with this process.
    fn index_of(&self, ph: *mut ProcessH) -> Option<usize> {
        use core::mem::size_of;

        let start = &self.mem[0] as *const ProcessH as usize;
        let ph = ph as *const ProcessH as usize;

        let result = ph.overflowing_sub(start);
        if result.1 {
            // Overflow occured - address is under the range of this frame.
            return None;
        }
        let offset = result.0;

        if offset >= self.mem.len() * size_of::<ProcessH>() {
            // Address is above the range of this frame.
            return None;
        }

        Some(offset / size_of::<ProcessH>())
    }

    /// Whether this process is in this frame.
    fn is_in_frame(&self, ph: *mut ProcessH) -> bool {
        let start = &self.mem[0] as *const ProcessH;
        let end = unsafe { start.offset(self.mem.len() as _) };

        let ph = ph as *const _;

        ph >= start && ph < end
    }
}

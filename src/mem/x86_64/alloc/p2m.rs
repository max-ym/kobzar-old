/// Stack of page addresses.
pub struct Stack {

    /// Top of the stack.
    top     : *mut u64,

    /// The count of addresses on the stack.
    count   : u32,
}

impl Stack {

    /// Remove last value from the stack and return it.
    fn pop(&mut self) -> Option<u64> {
        if self.count == 0 {
            return None;
        }

        unsafe {
            let val = *self.top;
            self.top = self.top.offset(-1);
            self.count -= 1;

            Some(val)
        }
    }
}

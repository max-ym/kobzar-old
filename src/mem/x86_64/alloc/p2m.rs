/// Stack of page addresses.
pub struct Stack {

    /// Top of the stack.
    top     : *mut Page2mHandle,

    /// The count of addresses on the stack.
    count   : u32,
}

#[derive(Clone, Copy)]
/// 2MiB page handle.
pub struct Page2mHandle {

    /// Address of a page.
    addr    : u64
}

impl Stack {

    /// Remove last value from the stack and return it.
    pub fn pop(&mut self) -> Option<Page2mHandle> {
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

    /// Add new value onto the stack.
    pub fn push(&mut self, val: Page2mHandle) {
        unsafe {
            self.count += 1;
            self.top = self.top.offset(1);
            *self.top = val;
        }
    }
}

impl Page2mHandle {

    pub fn new(addr: u64) -> Self {
        Page2mHandle { addr : addr }
    }

    pub fn addr(&self) -> u64 {
        self.addr
    }
}

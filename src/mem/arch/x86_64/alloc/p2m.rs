/// Stack of page addresses.
pub struct Stack {

    /// Top of the stack.
    top     : *mut Page2m,

    /// The count of addresses on the stack.
    count   : u32,
}

/// Set of pages. Used to store allocated pages handles.
/// Currently, set has size of 4KiB. Thus it can be put in a single
/// 4KiB page. Set can contain pages that address up to 1GiB of memory
/// without hash collisions of set entries.
#[repr(packed)]
pub struct Set2m {
    arr: [u64; 512]
}

#[derive(Clone, Copy)]
/// 2MiB page handle.
pub struct Page2m {

    /// Address of a page.
    addr    : u64
}

impl Stack {

    /// Remove last value from the stack and return it.
    pub fn pop(&mut self) -> Option<Page2m> {
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
    pub fn push(&mut self, val: Page2m) {
        unsafe {
            self.count += 1;
            self.top = self.top.offset(1);
            *self.top = val;
        }
    }
}

impl Page2m {

    pub fn new(addr: u64) -> Self {
        Page2m { addr : addr }
    }

    pub fn addr(&self) -> u64 {
        self.addr
    }
}

impl Set2m {

    /// Create an empty set.
    pub fn new() -> Self {
        Set2m {
            arr: [0; 512]
        }
    }
}

/// Stack of page addresses.
pub struct Stack {

    /// Top of the stack.
    top     : *mut Page2m,

    /// The count of addresses on the stack.
    count   : u32,
}

#[derive(Clone, Copy, PartialEq)]
/// 2MiB page handle.
pub struct Page2m {

    /// Address of a page.
    addr    : u64
}

/// 2MiB page ranges.
pub struct Range {
    bot     : u64,
    top     : u64,
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

impl Range {

    /// Create new range.
    pub fn new(top: u64, bottom: u64) -> Self {
        Range {
            top : top,
            bot : bottom,
        }
    }

    /// How many entries this range contains.
    pub fn length(&self) -> u64 {
        (self.top - self.bot) / 0x200000
    }

    /// Get index of a page status entry for this page address.
    ///
    /// # Safety
    /// It is expected that given absolute address of a page is
    /// within this range.
    pub unsafe fn abs_to_index(&self, absolute: u64) -> u64 {
        (absolute - self.bot) / 0x200000
    }

    /// Whether this page is within the range.
    pub fn contains(&self, page: Page2m) -> bool {
        let addr = page.addr();
        self.bot >= addr && self.top <= addr
    }

    /// Top value of the range.
    pub fn top(&self) -> u64 {
        self.top
    }

    /// Bottom value of the range.
    pub fn bottom(&self) -> u64 {
        self.bot
    }
}

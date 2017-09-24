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

/// Metadata for page that is needed to allocate and free it.
pub struct Page2mStatus {

    /// Original page data.
    page    : Page2m,

    /// How many tables use this page currently.
    used    : u32,
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

impl Page2mStatus {

    /// Create new status for given page.
    pub fn new(page: Page2m) -> Self {
        Page2mStatus {
            page    : page,
            used    : 0,
        }
    }

    /// How many tables use this page.
    pub fn use_count(&self) -> u32 {
        self.used
    }

    /// Notify that one more table uses this page now.
    /// Increments user counter.
    ///
    /// Returns new user counter value.
    pub fn inc_user(&mut self) -> u32 {
        self.used += 1;
        self.used
    }

    /// Notify that one table released this page now.
    /// Decrements user counter.
    ///
    /// Returns new user counter value.
    pub fn dec_user(&mut self) -> u32 {
        self.used -= 1;
        self.used
    }

    /// Set given user counter value.
    pub fn set_user(&mut self, val: u32) {
        self.used = val;
    }

    /// Whether this page is allocated for some page table.
    ///
    /// Opposite to fn `is_free`.
    pub fn is_used(&self) -> bool {
        self.used > 0
    }

    /// Whether this page is free to be allocated.
    ///
    /// Opposite to fn `is_used`.
    pub fn is_free(&self) -> bool {
        self.used == 0
    }

    /// Address of a page that this status was created for.
    pub fn page_address(&self) -> u64 {
        self.page.addr
    }

    /// Original page data.
    pub fn page(&self) -> &Page2m {
        &self.page
    }
}

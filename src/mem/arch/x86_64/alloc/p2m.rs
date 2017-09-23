/// Stack of page addresses.
pub struct Stack {

    /// Top of the stack.
    top     : *mut Page2m,

    /// The count of addresses on the stack.
    count   : u32,
}

/// Load factor for hash set - Set2m.
const SET2M_LOAD: usize = 512;

/// Set of pages. Used to store allocated pages handles.
/// Currently, set has size of 4KiB. Thus it can be put in a single
/// 4KiB page. Set can contain pages that address up to 1GiB of memory
/// without hash collisions of set entries.
#[repr(packed)]
pub struct Set2m {
    arr: [u64; SET2M_LOAD]
}

/// Entry of Set2m.
pub struct Set2mEntry {
    page    : Page2m,
    next    : *mut Set2mEntry,
}

#[derive(Clone, Copy, PartialEq)]
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
            arr: [0; SET2M_LOAD]
        }
    }

    /// Get hash for given page.
    fn page_to_hash(p: &Page2m) -> usize {
        let addr = p.addr() as usize;
        let page_size = 2048 * 2014;

        (addr / page_size) % SET2M_LOAD
    }

    /// Insert 2 MiB page to this set. The page is returned
    /// back if it is already in the set.
    pub fn insert(&mut self, page: Page2m) -> Option<Page2m> {
        unimplemented!();
    }
}

impl Set2mEntry {

    /// Create new set entry for given page.
    pub fn new(page: Page2m) -> Self {
        Set2mEntry {
            page: page,
            next: ::core::ptr::null_mut(),
        }
    }

    /// Check if any entry in the entry chain starting from current entry
    /// contains given page.
    pub fn any_contains(&self, page: &Page2m) -> Option<&Self> {
        let mut ptr = self as *const Self;

        while ptr as usize != 0 {
            unsafe {
                if (*ptr).page == *page {
                    return Some(&*ptr);
                }

                ptr = (*ptr).next;
            }
        }

        None
    }
}

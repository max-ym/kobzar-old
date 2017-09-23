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

    /// Get reference to entry with given hash if any.
    fn get_entry(&self, hash: usize) -> Option<&Set2mEntry> {
        if self.arr[hash] == 0 {
            return None;
        }

        let ptr = self.arr[hash] as *const Set2mEntry;
        unsafe { Some(&*ptr) }
    }

    /// Check if set contains given page.
    /// Return true if set really contains this page and
    /// false otherwise.
    pub fn contains(&self, page: &Page2m) -> bool {
        let hash = Self::page_to_hash(page);

        let option = self.get_entry(hash);
        if option.is_none() {
            return false;
        }
        let entry = option.unwrap();

        match entry.any_contains(page) {
            Some(_) => true,
            None    => false,
        }
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

    /// Get next entry reference if any.
    pub fn next(&self) -> Option<&Self> {
        if self.has_next() {
            unsafe {
                Some(&*self.next)
            }
        } else {
            None
        }
    }

    /// Check if this entry has pointer to next one.
    pub fn has_next(&self) -> bool {
        self.next as usize != 0
    }

    /// Check if any entry in the entry chain starting from current entry
    /// contains given page.
    pub fn any_contains(&self, page: &Page2m) -> Option<&Self> {
        if self.page == *page {
            return Some(self);
        }

        let mut r = self;
        while r.has_next() {
            r = r.next().unwrap();
            if r.page == *page {
                return Some(r);
            }
        }
        None
    }
}

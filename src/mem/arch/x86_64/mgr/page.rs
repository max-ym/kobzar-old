use core::ops::Range;
use mem::Address;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
/// 2MiB page wrap.
pub struct Page2m {

    /// Address of a page.
    addr    : Address
}

/// Infinite iterator over 2 MiB pages.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Page2mIter {

    page    : Page2m,
}

/// Iterator over 2 MiB page range.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Page2mRangeIter {

    start   : Page2m,
    end     : Page2m,

    cur     : Page2m,
}

impl Page2m {

    pub const SIZE: usize = 2 * 1024 * 1024;

    /// Create new Page2m wrap without checking if given address is a
    /// valid 2 MiB page address. Valid address must be aligned to
    /// 2 MiB boundary.
    pub unsafe fn new_unchecked(addr: Address) -> Self {
        Page2m { addr : addr }
    }

    /// Create new Page2m wrap. None is returned if address is not aligned
    /// and thus cannot point to a valid 2 MiB page.
    pub fn new(addr: Address) -> Option<Self> {
        if addr % Self::SIZE != 0 {
            None
        } else {
            let s = unsafe { Self::new_unchecked(addr) };
            Some(s)
        }
    }

    /// Address of a 2 MiB memory covered by this page wrapper.
    pub fn addr(&self) -> Address {
        self.addr
    }
}

impl Iterator for Page2mIter {

    type Item = Page2m;

    fn next(&mut self) -> Option<Self::Item> {
        let addr = self.page.addr + Page2m::SIZE;
        if addr < self.page.addr {
            None // Overflow
        } else {
            Some(Page2m { addr })
        }
    }
}

impl DoubleEndedIterator for Page2mIter {

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.page.addr < Page2m::SIZE {
            None
        } else {
            let addr = self.page.addr - Page2m::SIZE;
            Some(Page2m { addr })
        }
    }
}

impl Iterator for Page2mRangeIter {

    type Item = Page2m;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            return None;
        }

        let addr = self.cur.addr + Page2m::SIZE;
        self.cur.addr += Page2m::SIZE;
        Some(Page2m { addr })
    }
}

impl DoubleEndedIterator for Page2mRangeIter {

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.cur == self.start {
            return None;
        }

        let addr = self.cur.addr - Page2m::SIZE;
        self.cur.addr -= Page2m::SIZE;
        Some(Page2m { addr })
    }
}

impl ExactSizeIterator for Page2mRangeIter {

    fn len(&self) -> usize {
        let sub = self.end.addr - self.cur.addr;
        let sub: usize = sub.into();
        sub / Page2m::SIZE
    }
}

impl Page2mRangeIter {

    /// Create range iterator for given range.
    pub fn for_range(range: &Range<Page2m>) -> Self {
        Page2mRangeIter {
            start   : range.start,
            end     : range.end,

            cur     : range.start,
        }
    }
}

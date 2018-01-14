use mem::Address;

#[derive(Clone, Copy, PartialEq)]
/// 2MiB page wrap.
pub struct Page2m {

    /// Address of a page.
    addr    : Address
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

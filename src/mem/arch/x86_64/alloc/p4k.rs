use super::Page2m;

#[derive(Clone, Copy)]
pub struct Page4k {

    /// A base 2MiB page which contains this 4KiB page.
    base    : Page2m,

    /// Address of 4KiB memory region.
    addr    : u64,
}

impl Page4k {

    /// Get distance in 4KiB pages from the base address to this page
    /// (non inclusive).
    pub fn index(&self) -> u16 {
        let diff = self.addr - self.base.addr();
        (diff / 4096) as u16
    }

    pub fn new_by_index(base: Page2m, index: u16) -> Self {
        Page4k {
            base    : base.clone(),
            addr    : index as u64 * 4096 + base.addr()
        }
    }
}

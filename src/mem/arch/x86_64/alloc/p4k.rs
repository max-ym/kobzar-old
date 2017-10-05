use super::Page2m;

pub struct Page4kMap {

    /// A consumed 2MiB page that was split to 4KiB pages.
    base    : Page2m,

    /// A bitmap of used/free pages.
    ///
    /// 1 - is present,
    /// 0 - is abscent.
    map     : [u64; 8],
}

#[derive(Clone, Copy)]
pub struct Page4k {

    /// A base 2MiB page which contains this 4KiB page.
    base    : Page2m,

    /// Address of 4KiB memory region.
    addr    : u64,
}

impl Page4kMap {

    /// Create new 4KiB pages by splitting one 2MiB page.
    pub fn new(p2m: Page2m) -> Self {
        Page4kMap {
            base : p2m,
            map  : [0xFFFFFFFF_FFFFFFFF; 8],
        }
    }

    /// Get next free 4KiB page if any. Retrieved page is marked as used.
    pub fn get_next_page(&mut self) -> Option<Page4k> {
        let mut iter_counter = 0u64;

        while iter_counter < 8 {
            let qw = &mut self.map[iter_counter as usize];
            iter_counter += 1;

            let v = ::arch::bit::bsf_u64(*qw);
            if v.is_none() {
                // No free pages in this qword. Skip to next.
                continue;
            }
            // Found free page. Bit number of it in qword is stored here.
            let biti = v.unwrap();

            // Turn the page bit off = mark page abscent.
            *qw ^= 1 << biti;

            let page_index = biti + iter_counter * 64;

            return Some(Page4k::new_by_index(self.base, page_index as u16));
        }

        None
    }

    /// Whether given page was taken from this map.
    pub fn comprises(&self, page: &Page4k) -> bool {
        self.base.addr() == page.base.addr()
    }

    /// Get map indices from the page index. Return qword index and
    /// bit index in that qword.
    fn map_indices(page: &Page4k) -> (usize, u8) {
        let i = page.index();
        let mapi = i / 64;  // Map qword index.
        let biti = i % 64;  // Qword bit index.

        (mapi as usize, biti as u8)
    }

    /// Get page status in the map. Note that this fn does not check
    /// if page is comprised in this map and will not work valid if it
    /// is not.
    pub fn page_status(&self, page: &Page4k) -> PageStatus {
        let check_bit = |qword: u64, index: u8| -> bool {
            (qword & (1 << index)) != 0
        };

        let (mapi, biti) = Self::map_indices(page);

        match check_bit(self.map[mapi], biti) {
            true    => PageStatus::Present,
            false   => PageStatus::Abscent,
        }
    }

    /// Set page status to present. This function does not check if
    /// page is comprised by this map and will not work correct in case
    /// it is not comprised.
    fn set_page_present(&mut self, page: &Page4k) {
        let (mapi, biti) = Self::map_indices(page);
        self.map[mapi] |= 1 << biti;
    }

    /// Try to return page back into the map. This will consume the page
    /// and set it as free for use.
    pub fn try_return_page(&mut self, page: Page4k)
            -> Result<(), PageReturnErr> {
        use self::PageStatus::Present;
        use self::PageReturnErr::*;

        if !self.comprises(&page) {
            return Err(NotComprised);
        }

        if self.page_status(&page) == Present {
            return Err(AlreadyPresent);
        }

        self.set_page_present(&page);
        Ok(())
    }
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

#[derive(PartialEq)]
pub enum PageStatus {
    Present,
    Abscent,
}

#[derive(PartialEq)]
pub enum PageReturnErr {
    NotComprised,
    AlreadyPresent,
}

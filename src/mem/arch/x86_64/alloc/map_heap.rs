use super::pso::PageStatus;
use super::p4k::Page4k;
use super::p2m::Page2m;

/// Number of 4KiB pages in one split 2MiB page.
///
/// 2048 - 2MiB page size; divided by 4 - 4KiB page size that this bigger
/// page was split into; divided by 8 - bits count in one byte.
pub const P4KS_IN_P2M   : usize = 2048 / 4 / 8;

const PAGE_ALLOCATED    : bool = false;
const PAGE_FREE         : bool = true;

/// Qword to be used in bitmap.
#[repr(packed)]
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
struct Qword {
    pub val     : u64,
}

/// Struct that helps to find absolute allocated page address.
pub struct RelativeAddress {
    val     : usize,
}

/// Bitmap of allocated/free 4KiB pages.
#[repr(packed)]
pub struct Bitmap {
    /// Array of bytes of the bitmap.
    arr     : [Qword; P4KS_IN_P2M / 8],
}

/// 4KiB page status array heap entry.
pub struct HeapEntry {
    bitmap      : Bitmap,
    status_arr  : [PageStatus; P4KS_IN_P2M],
}

impl Default for Bitmap {

    fn default() -> Self {
        Bitmap {
            arr     : [Default::default(); P4KS_IN_P2M / 8]
        }
    }
}

impl Qword {

    /// Bit value by given index.
    pub fn bit(&self, index: usize) -> bool {
        self.val >> index != 0
    }

    /// Set bit by given index to specified value.
    pub fn set_bit(&mut self, index: usize, val: bool) {
        if val {
            self.val |= 1 << index;
        } else {
            self.val &= !(1 << index);
        }
    }
}

impl RelativeAddress {

    /// Create new relative address by given count of pages relative to
    /// base address of 2MiB page.
    pub fn new_by_count(count: usize) -> Self {
        RelativeAddress {
            val     : count
        }
    }

    /// Convert relative address to absolute by supplying base page.
    pub fn to_absolute(self, base: Page2m) -> Page4k {
        Page4k::new_by_index(base, self.val as _)
    }

    /// Count of pages relative to base address of 2MiB page.
    pub fn count(&self) -> usize {
        self.val
    }
}

impl Bitmap {

    /// Given bit value.
    pub fn bit(&self, index: usize) -> bool {
        let (qword_index, bit_index) = Self::index_split(index);
        self.arr[qword_index].bit(bit_index)
    }

    /// Set bit by given index to specified value.
    pub fn set_bit(&mut self, index: usize, val: bool) {
        let (qword_index, bit_index) = Self::index_split(index);
        self.set_qword_bit(qword_index, bit_index, val);
    }

    /// Set bit by given index to specified value.
    pub fn set_qword_bit
            (&mut self, qword_index: usize, bit_index: usize, val: bool) {
        self.arr[qword_index].set_bit(bit_index, val);
    }

    /// Split absolute bit index to index of qword that holds this bit and
    /// bit index in this qword.
    pub fn index_split(index: usize) -> (usize, usize) {
        let byte_index = index / 8;
        let qword_index = byte_index / 8;
        let bit_index = index % 64;

        (qword_index, bit_index)
    }

    /// Unite qword and it's bit indices into absolute index of bitmap bit.
    ///
    /// # Safety
    /// Does not check if provided indices are in bounds.
    pub unsafe fn unite_index(qword_index: usize, bit_index: usize) -> usize {
        qword_index * 64 + bit_index
    }

    /// Find first set bit and get it's indices. These are: first for qword
    /// which hold set bit and next is bit's index in this qword.
    pub fn first_set_bit(&self) -> Option<(usize, usize)> {
        unimplemented!()
    }
}

impl HeapEntry {

    /// Check if all 4KiB pages are free.
    pub fn is_free(&self) -> bool {
        unimplemented!()
    }

    fn first_free_page(&self) -> Option<(usize, usize)> {
        self.bitmap.first_set_bit()
    }

    /// Allocate new 4KiB page.
    pub fn alloc(&mut self) -> Option<RelativeAddress> {
        // Find set bit in bitmap.
        let set_bit = self.first_free_page();
        if set_bit.is_none() {
            return None;
        }
        let set_bit = set_bit.unwrap();

        let bit_index = unsafe { Bitmap::unite_index(set_bit.0, set_bit.1) };
        let rel_addr = RelativeAddress::new_by_count(bit_index);

        // Mark given page as used.
        self.bitmap.set_qword_bit(set_bit.0, set_bit.1, PAGE_ALLOCATED);
        self.status_arr[bit_index].inc_user();

        Some(rel_addr)
    }

    /// Deallocate 4KiB page. Change related bit in bitmap and set
    /// user counter to zero in related page status entry.
    ///
    /// # Safety
    /// Does not check whether this page is not used elsewere and
    /// forcely marks page as free.
    pub unsafe fn dealloc(&mut self, reladdr: RelativeAddress) {
        let bit_index = reladdr.count();
        self.bitmap.set_bit(bit_index, PAGE_FREE);
        self.status_arr[bit_index].set_user(0);
    }
}

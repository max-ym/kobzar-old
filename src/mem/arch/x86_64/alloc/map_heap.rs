// TODO remove. Module is probably messed up.

use super::PageStatus;
use super::Page4k;
use super::Page2m;

/// Number of 4KiB pages in one split 2MiB page.
///
/// 2048 - 2MiB page size; divided by 4 - 4KiB page size that this bigger
/// page was split into; divided by 8 - bits count in one byte.
pub const P4KS_IN_P2M   : usize = 2048 / 4 / 8;

const PAGE_ALLOCATED    : bool = false;
const PAGE_FREE         : bool = true;

/// Qword to be used in bitmap.
#[repr(packed)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Qword {
    pub val     : u64,
}

/// Struct that helps to find absolute allocated page address.
pub struct RelativeAddress {
    val     : usize,
}

/// Bitmap of allocated/free 4KiB pages.
#[repr(packed)]
struct Bitmap {
    /// Array of bytes of the bitmap.
    arr     : [Qword; P4KS_IN_P2M / 8],
}

/// 4KiB page status array heap entry.
pub struct HeapEntry {
    bitmap      : Bitmap,
    status_arr  : [PageStatus; P4KS_IN_P2M],
}

/// Map stores data about which cells of the array are used and which are
/// empty. Is used by Heap.
struct HeapMap {
    arr     : *mut Qword,
}

/// Array that stores heap entries. Used by the Heap.
struct HeapArray {

    /// Array that stores heap entries.
    arr     : *mut HeapEntry,

    /// Next heap entry that is free. Null when free entry is unknown.
    next_free   : *mut HeapEntry,

    /// Length of the array in bytes.
    byteslen    : u32,

    /// How many array entries are free.
    free    : u32,

    /// Map stores data about which cells of the array are used and which are
    /// empty.
    map     : HeapMap,
}

/// Heap of page maps that store 4KiB page status of divided 2MiB page.
pub struct Heap {

    /// Array that stores heap entries.
    arr     : HeapArray,
}

impl Default for Bitmap {

    fn default() -> Self {
        Bitmap {
            arr     : [Default::default(); P4KS_IN_P2M / 8]
        }
    }
}

impl Default for Qword {

    fn default() -> Self {
        Qword {
            val: !0
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

    /// Find the first set bit in this Qword.
    pub fn first_set_bit(&self) -> Option<u8> {
        unimplemented!()
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
    pub fn into_absolute(self, base: Page2m) -> Page4k {
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

impl HeapMap {

    pub fn set_bit(&mut self, index: usize, val: bool) {
        let ptrs = self.qword_by_bit_index_mut(index);
        ptrs.0.set_bit(ptrs.1, val);
    }

    /// Index of qword that stores given bit index. And index of a bit
    /// in the qword that corresponds to provided absolute index.
    fn qword_index(&self, bit_index: usize) -> (usize, usize) {
        let bits_in_qword = 64;
        (bit_index / bits_in_qword, bit_index % bits_in_qword)
    }

    /// Qword mutable reference with bit with given index.
    fn qword_by_bit_index_mut(&mut self, index: usize) -> (&mut Qword, usize) {
        let indices = self.qword_index(index);
        unsafe { (&mut *self.arr.offset(indices.0 as _), indices.1) }
    }
}

impl HeapArray {

    /// Allocate new heap entry in the heap array.
    ///
    /// # Safety
    /// Caller must ensure there is free space in the array. Otherwise
    /// behaviour is undefined.
    pub unsafe fn alloc(&mut self) -> &mut HeapEntry {
        use core::ptr::null_mut;
        if self.next_free == null_mut() {
            self.find_next_free();
        }

        // Next free entry
        let entry = &mut *self.next_free;
        self.next_free = null_mut();
        // TODO optimize

        // Save changes to bitmap.
        let index = self.ref_to_index(entry);
        self.map.set_bit(index, PAGE_ALLOCATED);

        // Update free pages counter.
        self.free -= 1;

        entry
    }

    /// Find next free entry and set HeapArray pointer to this value.
    fn find_next_free(&mut self) {
        unimplemented!()
    }

    /// Convert entry reference to the array index of that element in array.
    fn ref_to_index(&self, entry: &HeapEntry) -> usize {
        use core::mem::size_of;

        let entry_size = size_of::<HeapEntry>();
        let base = self.base();
        let addr = entry as *const _ as usize;

        (addr - base) / entry_size
    }

    /// Delete this entry from the array.
    ///
    /// # Safety
    /// Caller must ensure this entry exists. Otherwise behaviour undefined.
    pub unsafe fn drop(&mut self, entry_ref: &HeapEntry) {
        // Mark entry as free in heap map.
        let index = self.ref_to_index(entry_ref);
        self.map.set_bit(index, PAGE_FREE);

        // Update free pages counter.
        self.free += 1;
    }

    /// Check whether this array has free space.
    pub fn has_space(&self) -> bool {
        self.free != 0
    }

    /// Extend this array by given amount of bytes.
    ///
    /// # Safety
    /// Caller must ensure that array gets extended to free memory region.
    /// Otherwise some data may get corrupted.
    pub unsafe fn extend(&mut self, by: usize) {
        self.byteslen += by as u32;

        // TODO extend bitmap too.
        unimplemented!()
    }

    /// Address of the array.
    pub fn base(&self) -> usize {
        self.arr as usize
    }
}

impl Heap {

    /// Store given page in the heap.
    pub fn store(&mut self, page: Page2m) -> &mut HeapEntry {
        unimplemented!()
    }

    /// Remove given heap entry by it's reference.
    ///
    /// # Safety
    /// Ensure that given reference is created for the entry of this heap.
    /// Otherwise behaviour of the function is undefined.
    pub unsafe fn remove(&mut self, entry: &HeapEntry) {
        unimplemented!()
    }
}

use super::pso::PageStatus;

/// Number of 4KiB pages in one split 2MiB page.
///
/// 2048 - 2MiB page size; divided by 4 - 4KiB page size that this bigger
/// page was split into; divided by 8 - bits count in one byte.
pub const P4KS_IN_P2M: usize = 2048 / 4 / 8;

/// Qword to be used in bitmap.
#[repr(packed)]
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
struct Qword {
    pub val     : u64,
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

impl Bitmap {

    /// Given bit value.
    pub fn bit(&self, index: usize) -> bool {
        let (qword_index, bit_index) = Self::index_split(index);
        self.arr[qword_index].bit(bit_index)
    }

    /// Set bit by given index to specified value.
    pub fn set_bit(&mut self, index: usize, val: bool) {
        let (qword_index, bit_index) = Self::index_split(index);
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

    /// Find first set bit and get it's indices. These are: first for qword
    /// which hold set bit and next is bit's index in this qword.
    pub fn first_set_bit(&self) -> (usize, usize) {
        unimplemented!()
    }
}

impl HeapEntry {

    /// Check if all 4KiB pages are free.
    pub fn is_free(&self) -> bool {
        unimplemented!()
    }
}

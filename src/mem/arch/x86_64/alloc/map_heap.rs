use super::pso::PageStatus;

/// Number of 4KiB pages in one split 2MiB page.
///
/// 2048 - 2MiB page size; divided by 4 - 4KiB page size that this bigger
/// page was split into; divided by 8 - bits count in one byte.
pub const P4KS_IN_P2M: usize = 2048 / 4 / 8;

/// Bitmap of allocated/free 4KiB pages.
#[repr(packed)]
pub struct Bitmap {
    /// Array of bytes of the bitmap.
    pub arr     : [u8; P4KS_IN_P2M],
}

/// 4KiB page status array heap entry.
pub struct HeapEntry {
    bitmap      : Bitmap,
    status_arr  : [PageStatus; P4KS_IN_P2M],
}

impl Default for Bitmap {

    fn default() -> Self {
        Bitmap {
            arr     : [0; P4KS_IN_P2M]
        }
    }
}

impl HeapEntry {

    /// Check if all 4KiB pages are free.
    pub fn is_free(&self) -> bool {
        unimplemented!()
    }
}

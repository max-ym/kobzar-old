/// Bitmap of allocated/free 4KiB pages.
#[repr(packed)]
pub struct Bitmap {
    /// Array of bytes of the bitmap. Bytes count is:
    ///
    /// 2048 - 2MiB page size; divided by 4 - 4KiB page size that this bigger
    /// page was split into; divided by 8 - bits count in one byte.
    pub arr     : [u8; 2048 / 4 / 8],
}

impl Default for Bitmap {

    fn default() -> Self {
        Bitmap {
            arr     : [0; 64]
        }
    }
}

use super::Page2m;

/// Page Status. Holds the state of individual 2MiB pages.
/// Stores whether it is allocated or free.
pub struct PageStatus {
    /// Counter of how many table entries contain this page.
    /// When counter is zero, this page is free.
    used    : u32,
}

/// Page ranges.
struct Range {
    pub low     : u64,
    pub hi      : u64,
}

pub struct PSOArray {
    range   : Range,
    arr     : *mut PageStatus,
}

impl Range {

    /// Create new range.
    pub fn new(top: u32, bottom: u32) -> Self {
        Range {
            hi  : top,
            low : bottom,
        }
    }

    /// How many entries this range contains.
    pub fn length(&self) -> u64 {
        (self.hi - self.low) / 0x200000
    }

    /// Get index of a page status entry for this page address.
    ///
    /// # Safety
    /// It is expected that given absolute address of a page is
    /// within this range.
    pub unsafe fn abs_to_index(&self, absolute: u64) -> u64 {
        (absolute - self.low) / 0x200000
    }
}

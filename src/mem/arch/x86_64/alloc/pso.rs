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

#![allow(dead_code)]

/// The Code Segment Descriptor.
pub struct CodeSegmentDescriptor {
    a   : u32,
    _b  : u32, // This one is unused.
}

/// Code Segment Descriptor flag list.
#[repr(u32)]
enum CsdFlag {

    Present         = 1 << 15,

    Available       = 1 << 20,
    LongMode        = 1 << 21,
    DefaultBit      = 1 << 22,
    Granularity     = 1 << 23,
}

/// Use this macro to create a getter for CSD bit.
macro_rules! getter {
    ($n: tt, $x:tt) => {
        #[inline(always)]
        pub fn $n(&self) -> bool {
            self.a & (CsdFlag::$x as u32) != 0
        }
    };
}

impl CodeSegmentDescriptor {

    getter!(present         , Present);
    getter!(available       , Available);
    getter!(long_mode       , LongMode);
    getter!(default         , DefaultBit);
    getter!(granularity     , Granularity);
}


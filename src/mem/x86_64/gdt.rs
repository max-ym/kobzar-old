/// Descriptor Privilege Level.
#[repr(u32)]
pub enum Dpl {
    Dpl0 = 0,
    Dpl1 = 1,
    Dpl2 = 2,
    Dpl3 = 3,
}

/// The Code Segment Descriptor.
#[repr(packed)]
pub struct CodeSegmentDescriptor {
    a   : u32,
    _b  : u32, // This one is unused.
}

/// Code Segment Descriptor flag list.
#[repr(u32)]
enum CsdFlag {

    Accessed        = 1 << 8,
    Readable        = 1 << 9,
    Conforming      = 1 << 10,

    Present         = 1 << 15,

    Available       = 1 << 20,
    LongMode        = 1 << 21,
    DefaultBit      = 1 << 22,
    Granularity     = 1 << 23,

    Dpl             = (1 << 13) | (1 << 14),
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

    getter!(accessed        , Accessed);
    getter!(readable        , Readable);
    getter!(conforming      , Conforming);
    getter!(present         , Present);
    getter!(available       , Available);
    getter!(long_mode       , LongMode);
    getter!(default         , DefaultBit);
    getter!(granularity     , Granularity);

    pub fn dpl(&self) -> Dpl {
        let val = self.a & (CsdFlag::Dpl as u32) >> 13;
        unsafe { ::core::mem::transmute(val) }
    }
}


use super::{Dpl, Ist};

/// The list of architecture defined interrupt vectors.
/// For more information see Intel System Programming Guide.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum InterruptVector {

    DivideError     = 0,
    DebugException  = 1,
    Nmi             = 2,
    Breakpoint      = 3,
    Overflow        = 4,
    BoundRange      = 5,
    InvalidOpcode   = 6,
    NoMath          = 7,
    DoubleFault     = 8,

    InvalidTss          = 10,
    SegmentNotPresent   = 11,
    StackSegmentFault   = 12,
    GeneralProtection   = 13,
    PageFault           = 14,

    MathFault               = 16,
    AlignmentCheck          = 17,
    MachineCheck            = 18,
    SimdException           = 19,
    VirtualizationException = 20,
}

/// The structure of the trap gate.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TrapGate {

    /// First 16 bits of offset.
    offset0     : u16,

    /// Segment selector.
    segsel      : u16,

    flags       : u16,

    /// Bits 16-31 of offset.
    offset1     : u16,

    /// Bits 32-63 of offset.
    offset2     : u32,

    _reserved   : u32,
}

/// The structure of the interrupt gate.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct InterruptGate {

    /// First 16 bits of offset.
    offset0     : u16,

    /// Segment selector.
    segsel      : u16,

    flags       : u16,

    /// Bits 16-31 of offset.
    offset1     : u16,

    /// Bits 32-63 of offset.
    offset2     : u32,

    _reserved   : u32,
}

pub trait Gate {

    /// Address of the function that handles the interrupt.
    /// Intel System Programming Manual calls it 'offset'.
    fn offset(&self) -> u64;

    /// Set the address of the function that handles the interrupt.
    unsafe fn set_offset(&mut self, offset: u64);

    /// Segment Selector.
    fn segsel(&self) -> u16;

    unsafe fn set_segsel(&mut self, ss: u16);

    /// Interrupt Stack Table.
    fn ist(&self) -> Ist;

    unsafe fn set_ist(&mut self, ist: Ist);

    /// Present flag value.
    fn present(&self) -> bool;

    /// Change present flag value.
    unsafe fn set_present(&mut self, v: bool);

    /// Descriptor Privilege Level.
    fn dpl(&self) -> Dpl;

    unsafe fn set_dpl(&mut self, dpl: Dpl);

    /// Get all flags.
    fn flags(&self) -> u16;

    /// Set all flags with given value. Does not check if value is correct nor
    /// change any of it's bit. Even if some bits must be zero (but are set).
    unsafe fn set_flags(&mut self, f: u16);
}

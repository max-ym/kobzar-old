/// General interrupt descriptor table gate.
#[repr(packed)]
pub struct IDTGate(i64, i64);

/// Interrupt descriptor table
#[repr(packed)]
pub struct IDT {

    /// The array of all 256 gates of the IDT.
    pub gates:  [IDTGate; 256],
}

/// The list of architecture defined interrupt vectors.
/// For more information see Intel System Programming Guide.
pub enum InterruptVector {

    DivideError     = 0,
    DebugException  = 1,
    NMI             = 2,
    Breakpoint      = 3,
    Overflow        = 4,
    BOUNDRange      = 5,
    InvalidOpcode   = 6,
    NoMath          = 7,
    DoubleFault     = 8,

    InvalidTSS          = 10,
    SegmentNotPresent   = 11,
    StackSegmentFault   = 12,
    GeneralProtection   = 13,
    PageFault           = 14,

    MathFault               = 16,
    AlignmentCheck          = 17,
    MachineCheck            = 18,
    SIMDException           = 19,
    VirtualizationException = 20,
}

impl<'a> IDT {

    /// Get architecture defined interrupt gate.
    pub fn arch_gate(v: InterruptVector) -> &'a IDTGate {
        Self::get_idt_gate_at(v as u8)
    }

    /// Get interrupt gate at given position.
    pub fn idt_gate_at(position: u8) -> &'a IDTGate {
        unimplemented!();
    }
}

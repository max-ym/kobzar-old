/// General interrupt descriptor table gate.
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IDTGate(i64, i64);

/// Interrupt descriptor table
#[repr(packed)]
pub struct IDT {

    /// The array of all 256 gates of the IDT.
    pub gates:  [IDTGate; 256],
}

/// The list of architecture defined interrupt vectors.
/// For more information see Intel System Programming Guide.
#[derive(Copy, Clone)]
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

/// The register that holds interrupt descriptor table.
struct IDTR;

/// The value stored in IDTR register.
#[repr(packed)]
#[derive(Copy, Clone)]
struct IDTRValue {
    pub base    : i64,
    pub limit   : u16,
}

impl IDTRValue {

    pub fn mix(base: i64, limit: u16) -> Self {
        IDTRValue { base: base, limit: limit }
    }
}

impl IDTR {

    pub unsafe fn set(&self, idt_ptr: *const IDT, entry_count: u8) {
        let address = idt_ptr as i64;
        let val = IDTRValue::mix(address, entry_count as u16 * 16);
        asm!(
            "lidt   [$0]"
            : // No outputs
            : "r" (&val as *const IDTRValue as i64)
            :: "intel"
        );
    }

    pub fn get(&self) -> IDTRValue {
        let mut val: IDTRValue;
        unsafe { asm!(
            "sidt   [$0]"
            : "=r" (val)
            :: // No inputs
            : "intel"
        ); }
        val
    }
}

impl IDT {

    /// Get architecture defined interrupt gate.
    pub fn arch_gate(&self, v: InterruptVector) -> IDTGate {
        self.idt_gate_at(v as u8)
    }

    /// Get interrupt gate at given position.
    pub fn idt_gate_at(&self, position: u8) -> IDTGate {
        self.gates[position as usize]
    }

    pub unsafe fn overwrite_idt_gate_at
            (&mut self, position: u8, gate: IDTGate) {
        self.gates[position as usize] = gate;
    }

    pub unsafe fn overwrite_arch_gate_at
            (&mut self, v: InterruptVector, gate: IDTGate) {
        self.overwrite_idt_gate_at(v as u8, gate)
    }
}

/// The structure of the trap/interrupt gate.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TrapGate {

    /// First 16 bits of offset.
    pub offset0     : i16,

    /// Segment selector.
    pub segsel      : i16,

    pub flags       : i16,

    /// Bits 16-31 of offset.
    pub offset1     : i16,

    /// Bits 32-63 of offset.
    pub offset2     : i32,

    pub _reserved   : i32,
}

impl From<IDTGate> for TrapGate {

    fn from(gate: IDTGate) -> Self {
        unsafe { ::core::mem::transmute_copy(&gate) }
    }
}

impl Into<IDTGate> for TrapGate {

    fn into(self) -> IDTGate {
        unsafe { ::core::mem::transmute_copy(&self) }
    }
}

/// Interrupt Stack Table.
#[repr(u16)]
pub enum IST {
    IST0 = 0,
    IST1 = 1,
    IST2 = 2,
    IST3 = 3,
}

impl TrapGate {

    /// Get offset of the gate.
    pub fn offset(&self) -> i64 {
        let mut val: i64 = 0;

        val |= (self.offset0 as i64) << 0x00;
        val |= (self.offset1 as i64) << 0x10;
        val |= (self.offset2 as i64) << 0x20;
        val
    }

    /// Set offset of the gate.
    pub fn set_offset(&mut self, offset: i64) {
        self.offset0 = (offset >> 0x00) as i16;
        self.offset1 = (offset >> 0x10) as i16;
        self.offset2 = (offset >> 0x20) as i32;
    }

    /// Get segment selector of the gate.
    pub fn segment_selector(&self) -> i16 {
        self.segsel
    }

    /// Set the segment selector of the gate.
    pub fn set_segment_selector(&mut self, segsel: i16) {
        self.segsel = segsel;
    }

    pub fn ist(&self) -> IST {
        unsafe { ::core::mem::transmute(self.flags & 0b00000000_00000011) }
    }
}

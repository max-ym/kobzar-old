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
#[repr(u8)]
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

/// The structure of the trap gate.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TrapGate {

    /// First 16 bits of offset.
    pub offset0     : u16,

    /// Segment selector.
    pub segsel      : u16,

    pub flags       : u16,

    /// Bits 16-31 of offset.
    pub offset1     : u16,

    /// Bits 32-63 of offset.
    pub offset2     : u32,

    pub _reserved   : u32,
}

/// The structure of the interrupt gate.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct InterruptGate {

    /// First 16 bits of offset.
    pub offset0     : u16,

    /// Segment selector.
    pub segsel      : u16,

    pub flags       : u16,

    /// Bits 16-31 of offset.
    pub offset1     : u16,

    /// Bits 32-63 of offset.
    pub offset2     : u32,

    pub _reserved   : u32,
}

impl From<IDTGate> for TrapGate {

    fn from(gate: IDTGate) -> Self {
        let mut gate: TrapGate = unsafe { ::core::mem::transmute(gate) };
        // Set trap gate type flags.
        gate.flags &= 0b1110_0000_0001_1111;
        gate.flags |= 0b0000_1111_0000_0000;
        gate
    }
}

impl Into<IDTGate> for TrapGate {

    fn into(self) -> IDTGate {
        unsafe { ::core::mem::transmute(self) }
    }
}

impl From<IDTGate> for InterruptGate {

    fn from(gate: IDTGate) -> Self {
        let mut gate: InterruptGate = unsafe {
            ::core::mem::transmute(gate)
        };
        // Set interrupt gate type flags.
        gate.flags &= 0b1110_0000_0001_1111;
        gate.flags |= 0b0000_1110_0000_0000;
        gate
    }
}

impl Into<IDTGate> for InterruptGate {

    fn into(self) -> IDTGate {
        unsafe { ::core::mem::transmute(self) }
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

/// Descriptor Privilege Level.
#[repr(u16)]
pub enum DPL {
    DPL0 = 0,
    DPL1 = 1,
    DPL2 = 2,
    DPL3 = 3,
}

impl TrapGate {

    /// Get offset of the gate.
    pub fn offset(&self) -> u64 {
        let mut val: u64 = 0;

        val |= (self.offset0 as u64) << 0x00;
        val |= (self.offset1 as u64) << 0x10;
        val |= (self.offset2 as u64) << 0x20;
        val
    }

    /// Set offset of the gate.
    pub fn set_offset(&mut self, offset: u64) {
        self.offset0 = (offset >> 0x00) as u16;
        self.offset1 = (offset >> 0x10) as u16;
        self.offset2 = (offset >> 0x20) as u32;
    }

    /// Get segment selector of the gate.
    pub fn segment_selector(&self) -> u16 {
        self.segsel
    }

    /// Set the segment selector of the gate.
    pub fn set_segment_selector(&mut self, segsel: u16) {
        self.segsel = segsel;
    }

    pub fn ist(&self) -> IST {
        unsafe { ::core::mem::transmute(self.flags & 0b00000000_00000011) }
    }

    pub fn set_ist(&mut self, ist: IST) {
        // Clear IST bits
        self.flags &= 0b11111111_11111100;

        // Write new value
        self.flags |= ist as u16;
    }

    pub fn present(&self) -> bool {
        self.flags & 0b10000000_00000000 != 0
    }

    pub fn set_present(&mut self, p: bool) {
        match p {
            true  => self.flags |= 0b10000000_00000000,
            false => self.flags &= 0b01111111_11111111
        }
    }

    /// Get Descriptor Privilege Level.
    pub fn dpl(&self) -> DPL {
        unsafe { ::core::mem::transmute(self.flags & 0b01100000_00000000) }
    }

    /// Set Descriptor Privilege Level.
    pub fn set_dpl(&mut self, dpl: DPL) {
        // Clear old dpl bits.
        self.flags &= 0b10011_1111_1111_1111;

        // Set new dpl.
        self.flags |= (dpl as u16) << 13;
    }
}

impl InterruptGate {

    /// Get offset of the gate.
    pub fn offset(&self) -> u64 {
        // Use the same method from TrapGate.
        unsafe {
            ::core::mem::transmute_copy::<_, TrapGate>(self)
        }.offset()
    }

    /// Set offset of the gate.
    pub fn set_offset(&mut self, offset: u64) {
        // Use the same method from TrapGate.
        unsafe {
            ::core::mem::transmute_copy::<_, TrapGate>(self)
        }.set_offset(offset)
    }

    /// Get segment selector of the gate.
    pub fn segment_selector(&self) -> u16 {
        self.segsel
    }

    /// Set the segment selector of the gate.
    pub fn set_segment_selector(&mut self, segsel: u16) {
        self.segsel = segsel;
    }

    pub fn ist(&self) -> IST {
        // Use the same method from TrapGate.
        unsafe {
            ::core::mem::transmute_copy::<_, TrapGate>(self)
        }.ist()
    }

    pub fn set_ist(&mut self, ist: IST) {
        // Use the same method from TrapGate.
        unsafe {
            ::core::mem::transmute_copy::<_, TrapGate>(self)
        }.set_ist(ist)
    }

    pub fn present(&self) -> bool {
        // Use the same method from TrapGate.
        unsafe {
            ::core::mem::transmute_copy::<_, TrapGate>(self)
        }.present()
    }

    pub fn set_present(&mut self, p: bool) {
        // Use the same method from TrapGate.
        unsafe {
            ::core::mem::transmute_copy::<_, TrapGate>(self)
        }.set_present(p)
    }

    /// Get Descriptor Privilege Level.
    pub fn dpl(&self) -> DPL {
        // Use the same method from TrapGate.
        unsafe {
            ::core::mem::transmute_copy::<_, TrapGate>(self)
        }.dpl()
    }

    /// Set Descriptor Privilege Level.
    pub fn set_dpl(&mut self, dpl: DPL) {
        // Use the same method from TrapGate.
        unsafe {
            ::core::mem::transmute_copy::<_, TrapGate>(self)
        }.set_dpl(dpl)
    }
}

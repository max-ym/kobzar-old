use super::super::super::*;
use arch::regf;
use arch::xsave;
use mem::Address;

/// Context switch general purpose register file.
pub struct CsRegs {
    pub gp  : regf::GeneralPurpose,
    pub st  : regf::State,
}

/// Context switch data that must be saved/restored.
pub struct CsData {
    regs    : *mut CsRegs,

    /// XSAVE area address.
    xsave   : Address,

    /// Mask for XSAVE instructions that show which components must be
    /// saved for this process.
    xmask   : xsave::Mask,
}

pub struct ProcessH {
    id      : u32,
    csdata  : CsData,
    state   : ProcessState,
}

impl ProcessHandle for ProcessH {

    fn state(&self) -> ProcessState {
        self.state
    }

    fn id(&self) -> u32 {
        self.id
    }
}

impl ProcessH {

    /// General Purpose register file.
    pub fn gp_regs(&self) -> *const CsRegs {
        self.csdata.regs as _
    }

    /// General Purpose register file.
    pub fn gp_regs_mut(&mut self) -> *mut CsRegs {
        self.csdata.regs
    }

    /// Check whether this process uses general purpose registers.
    /// These registers may not be used in case this process
    /// is a halt process that causes processor to sleep.
    pub fn has_gp_regs(&self) -> bool {
        !self.csdata.regs.is_null()
    }

    /// Check if this process requires to save the state components.
    pub fn has_xsave(&self) -> bool {
        let mask: u64 = self.csdata.xmask.into();
        mask != 0
    }

    /// Address of XSAVE area.
    pub fn xsave_area(&self) -> Address {
        self.csdata.xsave
    }

    /// Mask for XSAVE.
    pub fn xsave_mask(&self) -> xsave::Mask {
        self.csdata.xmask
    }
}

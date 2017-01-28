/// Info read from MSR.
#[derive(Clone, Copy)]
pub struct Info {
    pub eax     : u32,
    pub edx     : u32,
}

impl Info {

    /// Get data in MSR by it's code. This function is unsafe as
    /// some MSRs may not be defined and so this call will cause
    /// General Protection fault. Ensure that MSR with given ID actually
    /// exists.
    pub unsafe fn read_by_id(id: u32) -> Info {
        let (a, d);
        asm!(
            "rdmsr"
            : "={eax}"(a), "={edx}"(d)
            : "{ecx}"(id)
        );

        Info { eax:a, edx:d }
    }

    /// See 'read_by_id'. Note, that this function generally must not be used.
    /// It is more appropriate to use relevant 'read' function in the structure
    /// that represents the desired MSR.
    pub unsafe fn read(msr: Msr) -> Info {
        Self::read_by_id(msr as u32)
    }
}

/// Model Specific Register ID list.
#[repr(u32)]
pub enum Msr {
    ApicBase        = 27,
}

macro_rules! derive_info {
    ($x:ident) => (
        #[derive(Clone, Copy)]
        pub struct $x {
            eax     : u32,
            edx     : u32,
        }

        impl Into<Info> for $x {

            fn into(self) -> Info {
                Info { eax: self.eax, edx: self.edx }
            }
        }

        impl $x {

            /// Read this given MSR. Note that if it is not defined
            /// in the processor, General Protection fault will be
            /// rised. You need to ensure that processor supports this MSR.
            pub unsafe fn read() -> Self {
                let info = Info::read(Msr::$x);
                // Convert the Info structure to correspond to given MSR.
                ::core::mem::transmute(info)
            }
        }
    );
}

derive_info!(ApicBase);

impl ApicBase {

    pub fn bsp(&self) -> bool {
        self.edx & 0b0000_0001_0000_0000 != 0
    }

    pub fn set_bsp(&mut self, val: bool) {
        match val {
            true  => self.edx |= 0b0000_0001_0000_0000,
            false => self.edx &= 0b1111_1110_1111_1111
        }
    }
}

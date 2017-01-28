/// Information stored by CPUID instruction in appropriate registers.
#[derive(Clone, Copy)]
pub struct Info {
    pub eax     : u32,
    pub ebx     : u32,
    pub ecx     : u32,
    pub edx     : u32,
}

/// Enum that stores all valid CPUID query codes.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum InfoType {
    VendorString    = 0x00,
    Features        = 0x01,
    TLB             = 0x02,
    Serial          = 0x03,

    IntelExtended       = 0x8000_0000,
    IntelFeatures       = 0x8000_0001,
    IntelBrandString    = 0x8000_0002,
    IntelBrandStringMore= 0x8000_0003,
    IntelBrandStringEnd = 0x8000_0004,
}

impl Info {

    /// Run CPUID instruction to query information.
    #[inline(always)]
    pub fn get(info: InfoType) -> Self {
        Self::get_by_code(info as u32)
    }

    #[inline(always)]
    pub fn get_by_code(request: u32) -> Self {
        let (a, b, c, d);

        unsafe { asm!(
            "cpuid"
            : "={eax}"(a), "={ebx}"(b), "={ecx}"(c), "={edx}"(d)
            : "{eax}"(request)
        ); }

        Info { eax:a, ebx:b, ecx:c, edx:d }
    }
}

macro_rules! derive_info {
    ($x:ident) => (
        #[derive(Clone, Copy)]
        pub struct $x {
            info    : Info
        }

        impl From<Info> for $x {

            fn from(i: Info) -> Self {
                $x { info:i }
            }
        }

        impl Into<Info> for $x {

            fn into(self) -> Info {
                self.info
            }
        }
    );
}

derive_info!(VendorString);

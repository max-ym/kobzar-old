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
    pub fn get(info: InfoType) -> Self {
        use self::InfoType::*;
        let (a, b, c, d);

        let i = info as u32;
        unsafe { asm!(
            "cpuid"
            : "={eax}"(a), "={ebx}"(b), "={ecx}"(c), "={edx}"(d)
            : "{eax}"(i)
        ); }

        Info { eax:a, ebx:b, ecx:c, edx:d }
    }
}

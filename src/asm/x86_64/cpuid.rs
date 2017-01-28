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
derive_info!(Features);
derive_info!(TLB);
derive_info!(Serial);
derive_info!(IntelExtended);
derive_info!(IntelFeatures);
derive_info!(IntelBrandString);
derive_info!(IntelBrandStringMore);
derive_info!(IntelBrandStringEnd);

impl VendorString {

    pub fn vendor(&self, s: &mut [char; 13]) {
        s[12] = '\0'; // Null-terminate the string.

        s[0x00] = ((self.info.ebx & 0x000000FF) >> 0x00) as u8 as char;
        s[0x01] = ((self.info.ebx & 0x0000FF00) >> 0x08) as u8 as char;
        s[0x02] = ((self.info.ebx & 0x00FF0000) >> 0x10) as u8 as char;
        s[0x03] = ((self.info.ebx & 0xFF000000) >> 0x18) as u8 as char;
        s[0x04] = ((self.info.edx & 0x000000FF) >> 0x00) as u8 as char;
        s[0x05] = ((self.info.edx & 0x0000FF00) >> 0x08) as u8 as char;
        s[0x06] = ((self.info.edx & 0x00FF0000) >> 0x10) as u8 as char;
        s[0x07] = ((self.info.edx & 0xFF000000) >> 0x18) as u8 as char;
        s[0x08] = ((self.info.ecx & 0x000000FF) >> 0x00) as u8 as char;
        s[0x09] = ((self.info.ecx & 0x0000FF00) >> 0x08) as u8 as char;
        s[0x0A] = ((self.info.ecx & 0x00FF0000) >> 0x10) as u8 as char;
        s[0x0B] = ((self.info.ecx & 0xFF000000) >> 0x18) as u8 as char;
    }

    /// Maximal input value for basic CPUID information.
    pub fn max_value(&self) -> u32 {
        self.info.eax
    }
}

/// Information stored by CPUID instruction in appropriate registers.
#[derive(Clone, Copy)]
pub struct Info {
    pub eax     : u32,
    pub ebx     : u32,
    pub ecx     : u32,
    pub edx     : u32,
}

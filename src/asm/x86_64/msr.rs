/// Info read from MSR.
#[derive(Clone, Copy)]
pub struct Info {
    pub eax     : u64,
    pub edx     : u64,
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
            : "={rax}"(a), "={rdx}"(d)
            : "{ecx}"(code)
        );

        Info { eax:a, edx:d }
    }
}

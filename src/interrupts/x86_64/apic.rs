/// The base address of Local APIC registers mapped to RAM.
/// This struct allows to update the value atomically. It is designed
/// to be used as a singleton.
pub struct BaseAddress {
    lock    : ::core::sync::atomic::AtomicBool,
    addr    : u64,
}

pub static mut BASE_ADDRESS: BaseAddress
        = BaseAddress {
            // This initial value is invalid. At system startup must be
            // loaded with the right value.
            addr: 0,
            lock: ::core::sync::atomic::AtomicBool::new(false)
        };

/// Local APIC related functions and data.
pub struct LocalApic {
    // TODO
}

impl LocalApic {

    pub fn is_present() -> bool {
        ::asm::cpuid::Features::get().local_apic_is_present()
    }
}

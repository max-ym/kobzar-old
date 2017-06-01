//use ::interrupts::apic;

pub fn setup_interrupts() {
    use ::early::{LoggerTrait, logger};
    logger().println("Starting Local APIC, if any available.");
    //apic::init();

    // TODO
}

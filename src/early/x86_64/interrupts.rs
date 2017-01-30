use ::interrupts::apic;

pub fn setup_interrupts(logger: &mut ::early::Logger) {
    use super::LoggerTrait;
    logger.println("Starting Local APIC, if any available.");
    apic::init();

    // TODO
}

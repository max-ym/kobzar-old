use ::ccs;
use ::early::ccs::*;
use ::mem::map::{CCS_BASIC_SETUP_ADDRESS, CCS_BASIC_SETUP_ADDRESS_END};

/// Create new service and object lists so that CCS could work with them.
pub fn setup() {
    use core::mem::{size_of, transmute};
    unsafe {
        // Current used memory pointer.
        let mut memptr = CCS_BASIC_SETUP_ADDRESS;

        // Create Kobzar object.
        let machine: *mut ccs::Object = transmute(memptr);
        *machine = ccs::Object::new(MACHINE_ROOT_OBJECT);

        // Skip created object.
        memptr += size_of::<ccs::Object>();

        // Create Kobzar object.
        let kobzar: *mut ccs::Object = transmute(memptr);
        *kobzar = ccs::Object::new(KOBZAR_ROOT_OBJECT);

        memptr += size_of::<ccs::Object>();

        // Create kernel object.
        let kernel: *mut ccs::Object = transmute(memptr);
        *kernel = ccs::Object::new(KERNEL_OBJECT);

        memptr += size_of::<ccs::Object>();

        // Create RAM manager object.
        let ram: *mut ccs::Object = transmute(memptr);
        *ram = ccs::Object::new(RAM_MANAGER_OBJECT);

        memptr += size_of::<ccs::Object>();

        let allocate: *mut ccs::Service = transmute(memptr);
        *allocate = ccs::Service::new(RAM_ALLOCATE_SERVICE, 0 /* TODO */);

        memptr += size_of::<ccs::Service>();

        let release: *mut ccs::Service = transmute(memptr);
        *release =  ccs::Service::new(RAM_RELEASE_SERVICE, 0 /* TODO */);

        memptr += size_of::<ccs::Service>();

        unimplemented!();
    }
}

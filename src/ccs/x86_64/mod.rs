use ::ccs;
use ::early::ccs::*;
use ::mem::map::{CCS_BASIC_SETUP_ADDRESS, CCS_BASIC_SETUP_ADDRESS_END};

#[derive(Clone, Copy)]
struct Ptr {
    addr: usize
}

impl Ptr {

    /// Create new pointer struct for given address.
    pub fn new(addr: usize) -> Self {
        Ptr { addr : addr }
    }

    pub fn as_object_ptr<'a>(&self) -> *mut ccs::Object<'a> {
        unsafe { ::core::mem::transmute(self.addr) }
    }

    pub fn as_service_node_ptr<'a>(&self) -> *mut ccs::ServiceListNode<'a> {
        unsafe { ::core::mem::transmute(self.addr) }
    }

    pub fn as_object_node_ptr<'a>(&self) -> *mut ccs::ObjectListNode<'a> {
        unsafe { ::core::mem::transmute(self.addr) }
    }

    pub fn skip_object(&mut self) {
        self.addr += ::core::mem::size_of::<ccs::Object>();
    }

    pub fn skip_service_node(&mut self) {
        self.addr += ::core::mem::size_of::<ccs::ServiceListNode>();
    }

    pub fn skip_object_node(&mut self) {
        self.addr += ::core::mem::size_of::<ccs::ObjectListNode>();
    }

    pub fn value(&self) -> usize {
        self.addr
    }

    pub fn next_object_ptr<'a>(&mut self) -> *mut ccs::Object<'a> {
        let object_ptr = self.as_object_ptr();
        self.skip_object();
        object_ptr
    }

    pub fn next_object_node_ptr<'a>(&mut self)
            -> *mut ccs::ObjectListNode<'a> {
        let list_node_ptr = self.as_object_node_ptr();
        self.skip_object_node();
        list_node_ptr
    }

    pub fn next_service_node_ptr<'a>(&mut self)
            -> *mut ccs::ServiceListNode<'a> {
        let list_node_ptr = self.as_service_node_ptr();
        self.skip_service_node();
        list_node_ptr
    }
}

/// Create new service and object lists so that CCS could work with them.
pub fn setup() {
    use core::mem::{size_of, transmute};

    // Current used memory pointer.
    let mut memptr = Ptr::new(CCS_BASIC_SETUP_ADDRESS);

    // Create objects and services.
    let machine = ccs::Object::new(MACHINE_ROOT_OBJECT);
    let kobzar  = ccs::Object::new(KOBZAR_ROOT_OBJECT);
    let kernel  = ccs::Object::new(KERNEL_OBJECT);
    let ram     = ccs::Object::new(RAM_MANAGER_OBJECT);

    let allocate = ccs::Service::new(RAM_ALLOCATE_SERVICE, 0 /* TODO */);
    let release  = ccs::Service::new(RAM_RELEASE_SERVICE, 0 /* TODO */);

    // Generate list nodes
    let ram_service_0   = ccs::ServiceListNode::new(allocate);
    let ram_service_1   = ccs::ServiceListNode::new(release);

    let kernel_objlist  = ccs::ObjectListNode::new(ram);
    let kobzar_objlist  = ccs::ObjectListNode::new(kernel);
    let machine_objlist = ccs::ObjectListNode::new(kobzar);

    // Create pointer list.
    let machine_ptr         = memptr.next_object_ptr();
    let ram_service_0_ptr   = memptr.next_service_node_ptr();
    let ram_service_1_ptr   = memptr.next_service_node_ptr();
    let kernel_objlist_ptr  = memptr.next_object_node_ptr();
    let kobzar_objlist_ptr  = memptr.next_object_node_ptr();
    let machine_objlist_ptr = memptr.next_object_node_ptr();

    // Ensure pointer does not break expected bounds.
    if memptr.value() >= CCS_BASIC_SETUP_ADDRESS_END {
        panic!("CCS table violates memory limits");
    }

    unsafe {
        // Save object and service handles in CCS table.
        *machine_ptr            = machine;
        *ram_service_0_ptr      = ram_service_0;
        *ram_service_1_ptr      = ram_service_1;
        *kernel_objlist_ptr     = kernel_objlist;
        *kobzar_objlist_ptr     = kobzar_objlist;
        *machine_objlist_ptr    = machine_objlist;

        unimplemented!();
    }
}

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

pub fn setup() {
    unimplemented!()
}

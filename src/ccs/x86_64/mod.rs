use ::ccs;
use ::early::ccs::*;
use ::mem::map::{CCS_BASIC_SETUP_ADDRESS, CCS_BASIC_SETUP_ADDRESS_END};

#[derive(Clone, Copy)]
struct Ptr {
    addr    : usize,
    limit   : usize,
}

impl Ptr {

    /// Create new pointer struct for given address.
    pub fn new_heap(addr: usize, limit: usize) -> Self {
        Ptr {
            addr    : addr,
            limit   : limit,
        }
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

    pub fn addr(&self) -> usize {
        self.addr
    }

    pub fn limit(&self) -> usize {
        self.limit
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

    pub fn exceeds(&self, addr: usize) -> bool {
        self.addr >= addr
    }
}

trait Allocate : Sized {

    fn get_allocated(ptr: &mut Ptr) -> Option<*mut Self> {
        let size = ::core::mem::size_of::<Self>();

        if ptr.addr() + size > ptr.limit() {
            None
        } else {
            unsafe { Some(Self::alloc_next_ptr(ptr)) }
        }
    }

    unsafe fn alloc_next_ptr(ptr: &mut Ptr) -> *mut Self;

    fn allocate_ptr(ptr: &mut Ptr) -> *mut Self {
        let option = Self::get_allocated(ptr);

        if let Some(val) = option {
            val
        } else {
            panic!("CCS table violates memory limits");
        }
    }

    fn allocate_mut(ptr: &mut Ptr) -> &mut Self {
        unsafe { &mut *Self::allocate_ptr(ptr) }
    }

    fn allocate_and_move(mut self, ptr: &mut Ptr) -> &mut Self {
        let reference = Self::allocate_mut(ptr);
        *reference = self;
        reference
    }
}

impl<'a> Allocate for ccs::Object<'a> {

    unsafe fn alloc_next_ptr(ptr: &mut Ptr) -> *mut Self {
        ptr.next_object_ptr()
    }
}

impl<'a> Allocate for ccs::ServiceListNode<'a> {

    unsafe fn alloc_next_ptr(ptr: &mut Ptr) -> *mut Self {
        ptr.next_service_node_ptr()
    }
}

impl<'a> Allocate for ccs::ObjectListNode<'a> {

    unsafe fn alloc_next_ptr(ptr: &mut Ptr) -> *mut Self {
        ptr.next_object_node_ptr()
    }
}

pub fn setup() {
    unimplemented!()
}

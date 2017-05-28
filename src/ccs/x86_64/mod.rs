use ::ccs;
use ::early::ccs::*;
use ::mem::map::{CCS_BASIC_SETUP_ADDRESS, CCS_BASIC_SETUP_ADDRESS_END};
use core::cell::Cell;

#[derive(Clone)]
struct Ptr {
    addr    : Cell<usize>,
    limit   : usize,
}

impl Ptr {

    /// Create new pointer struct for given address.
    pub fn new_heap(addr: usize, limit: usize) -> Self {
        Ptr {
            addr    : Cell::new(addr),
            limit   : limit,
        }
    }

    pub fn as_object_ptr(&self) -> *mut ccs::Object {
        unsafe { ::core::mem::transmute(self.addr.get()) }
    }

    pub fn as_service_node_ptr(&self) -> *mut ccs::ServiceListNode {
        unsafe { ::core::mem::transmute(self.addr.get()) }
    }

    pub fn as_object_node_ptr(&self) -> *mut ccs::ObjectListNode {
        unsafe { ::core::mem::transmute(self.addr.get()) }
    }

    pub fn skip_object(&self) {
        let size = ::core::mem::size_of::<ccs::Object>();
        self.addr.set(self.addr.get() + size);
    }

    pub fn skip_service_node(&self) {
        let size = ::core::mem::size_of::<ccs::ServiceListNode>();
        self.addr.set(self.addr.get() + size);
    }

    pub fn skip_object_node(&self) {
        let size = ::core::mem::size_of::<ccs::ObjectListNode>();
        self.addr.set(self.addr.get() + size);
    }

    pub fn addr(&self) -> usize {
        self.addr.get()
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn next_object_ptr(&self) -> *mut ccs::Object {
        let object_ptr = self.as_object_ptr();
        self.skip_object();
        object_ptr
    }

    pub fn next_object_node_ptr(&self) -> *mut ccs::ObjectListNode {
        let list_node_ptr = self.as_object_node_ptr();
        self.skip_object_node();
        list_node_ptr
    }

    pub fn next_service_node_ptr(&self) -> *mut ccs::ServiceListNode {
        let list_node_ptr = self.as_service_node_ptr();
        self.skip_service_node();
        list_node_ptr
    }

    pub fn exceeds(&self, addr: usize) -> bool {
        self.addr.get() >= addr
    }
}

trait Allocate : Sized {

    fn get_allocated(ptr: &Ptr) -> Option<*mut Self> {
        let size = ::core::mem::size_of::<Self>();

        if ptr.addr() + size > ptr.limit() {
            None
        } else {
            unsafe { Some(Self::alloc_next_ptr(ptr)) }
        }
    }

    unsafe fn alloc_next_ptr(ptr: &Ptr) -> *mut Self;

    fn allocate_ptr(ptr: &Ptr) -> *mut Self {
        let option = Self::get_allocated(ptr);

        if let Some(val) = option {
            val
        } else {
            panic!("CCS table violates memory limits");
        }
    }

    fn allocate_mut(ptr: &Ptr) -> &mut Self {
        unsafe { &mut *Self::allocate_ptr(ptr) }
    }

    fn allocate_and_move(mut self, ptr: &Ptr) -> &mut Self {
        let reference = Self::allocate_mut(ptr);
        *reference = self;
        reference
    }
}

impl Allocate for ccs::Object {

    unsafe fn alloc_next_ptr(ptr: &Ptr) -> *mut Self {
        ptr.next_object_ptr()
    }
}

impl Allocate for ccs::ServiceListNode {

    unsafe fn alloc_next_ptr(ptr: &Ptr) -> *mut Self {
        ptr.next_service_node_ptr()
    }
}

impl Allocate for ccs::ObjectListNode {

    unsafe fn alloc_next_ptr(ptr: &Ptr) -> *mut Self {
        ptr.next_object_node_ptr()
    }
}

pub fn setup() {
    use super::lists::{List, ListNode};

    let mut heap = Ptr::new_heap
            (CCS_BASIC_SETUP_ADDRESS, CCS_BASIC_SETUP_ADDRESS_END);

    let mut root_obj    = ccs::Object::new(MACHINE_ROOT_OBJECT);
    let mut kobzar_obj  = ccs::Object::new(KOBZAR_ROOT_OBJECT);
    let mut kernel_obj  = ccs::Object::new(KERNEL_OBJECT);
    let mut ram_mgr_obj = ccs::Object::new(RAM_MANAGER_OBJECT);

    // TODO: set valid service fn pointers.
    let allocate_serv   = ccs::Service::new(RAM_ALLOCATE_SERVICE, 0);
    let release_serv    = ccs::Service::new(RAM_RELEASE_SERVICE, 0);

    unsafe {
        let mut list_node = ccs::ObjectListNode::new(kobzar_obj);
        let kobzar_obj = list_node.elem_mut_ptr();
        root_obj.pub_obj_list.append(list_node.allocate_and_move(&heap));

        let mut list_node = ccs::ObjectListNode::new(kernel_obj);
        let kernel_obj = list_node.elem_mut_ptr();
        (*kobzar_obj).pub_obj_list.append(list_node.allocate_and_move(&heap));

        let mut list_node = ccs::ObjectListNode::new(ram_mgr_obj);
        let ram_mgr_obj = list_node.elem_mut_ptr();
        (*kernel_obj).pub_obj_list.append(list_node.allocate_and_move(&heap));

        let list_node = ccs::ServiceListNode::new(allocate_serv);
        (*ram_mgr_obj).pub_serv_list.append(list_node.allocate_and_move(&heap));

        let list_node = ccs::ServiceListNode::new(release_serv);
        (*ram_mgr_obj).pub_serv_list.append(list_node.allocate_and_move(&heap));
    }

    root_obj.allocate_and_move(&heap);
}

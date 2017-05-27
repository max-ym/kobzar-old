use super::*;

/// List of items.
pub trait List {

    /// Node type of the list that is used to store listed items.
    type Node;

    /// Top of the list. The first entry of it. It can be None if list
    /// is empty or a first node of the list.
    fn top(&self) -> Option<Self::Node>;

    /// Set the first node of the list. Note that each node has a pointer
    /// to next one. If you change the first node and it is pointing
    /// to another node than it was in the old node you may loose all
    /// old nodes that were connected by pointers to the old top one.
    /// If you just need to replace the first node without changing other
    /// part of the list you need another way.
    fn set_top(&mut self, top: Option<Self::Node>);
}

/// The node of the list. Stores only one element of the list and a
/// pointer to the next node if any.
pub trait ListNode<'a> {

    /// The type of the item that is stored in the list.
    type Item;

    /// Reference to the item in this node.
    fn elem_ref(&self) -> &Self::Item;

    /// Set the item in this node.
    fn set_elem(&mut self, top: Self::Item);

    /// Get mutable reference to the next node option.
    fn next_mut(&'a mut self) -> &'a mut Option<&'a Self>;

    /// Get a reference to the next node option.
    fn next_ref(&self) -> &Option<&'a Self>;
}

/// Handle to edit the item of the object or service lists in any object
/// of the CCS network. It stores information about a list in which the
/// item is stored, the object that contains the list with item and other
/// data that is needed to edit the item list or item itself.
pub trait Handle<'a> : Sized {

    /// Type of the element that can be edited.
    type Item;

    /// Type of the list node that can store Item type.
    type ListNode : ListNode<'a>;

    /// Reference to the item.
    fn item_ref(&'a self) -> &'a Self::Item;

    /// Mutable reference to the item.
    fn item_mut(&'a mut self) -> &'a mut Self::Item;

    /// Mutable reference of the mutable list node that stores
    /// the item.
    fn node_ptr_mut(&'a mut self) -> &'a mut *mut Self::ListNode;

    /// Pointer to the mutable list node that stores the current item.
    fn node_ptr(&'a self) -> *mut Self::ListNode;

    /// Get a mutable reference to the mutable list node pointer of the
    /// previous item if any.
    fn get_mut_prev_node_ptr(&'a mut self)
            -> &'a mut Option<*mut Self::ListNode>;

    /// Get a pointer to the mutable list node of the previous item if any.
    fn get_prev_node_ptr(&'a self) -> Option<*mut Self::ListNode>;

    /// Pointer to mutable list node of the next item if any.
    fn next_node_ptr(&'a self) -> Option<*mut Self::ListNode>;

    /// Reference of the list node of the next item if any.
    fn next_node_ref(&'a self) -> Option<&'a Self::ListNode> {
        unsafe {
            match self.next_node_ptr() {
                Some(val) => Some(&*val),
                None      => None
            }
        }
    }
}

trait HandleRemovable<'a> : Handle<'a> {

    fn is_in_list_top(&'a self) -> bool;

    fn set_next_node_as_list_top(&'a self);

    fn link_next_node_to_prev(&'a self);

    fn remove_from_list(&'a mut self) {
        if self.is_in_list_top() {
            self.set_next_node_as_list_top();
        } else {
            self.link_next_node_to_prev();
        }
    }
}

pub struct ServiceList<'a> {
    top : Option<&'a ServiceListNode<'a>>,
}

impl<'a> Default for ServiceList<'a> {

    fn default() -> Self {
        ServiceList { top : None }
    }
}

impl<'a> List for ServiceList<'a> {

    type Node = &'a ServiceListNode<'a>;

    fn top(&self) -> Option<Self::Node> {
        self.top
    }

    fn set_top(&mut self, top: Option<Self::Node>) {
        self.top = top;
    }
}

pub struct ServiceListNode<'a> {

    /// The actual service.
    service : Service<'a>,

    /// Next node, if any.
    next : Option<&'a ServiceListNode<'a>>,
}

impl<'a> ServiceListNode<'a> {

    pub fn new(service: Service<'a>) -> Self {
        ServiceListNode {
            service : service,
            next    : None,
        }
    }
}

impl<'a> ListNode<'a> for ServiceListNode<'a> {

    type Item = Service<'a>;

    fn elem_ref(&self) -> &Self::Item {
        &self.service
    }

    fn set_elem(&mut self, elem: Self::Item) {
        self.service = elem;
    }

    fn next_mut(&'a mut self) -> &'a mut Option<&'a Self> {
        &mut self.next
    }

    fn next_ref(&self) -> &Option<&'a Self> {
        &self.next
    }
}

pub struct ObjectList<'a> {
    top: Option<&'a ObjectListNode<'a>>,
}

impl<'a> Default for ObjectList<'a> {

    fn default() -> Self {
        ObjectList { top : None }
    }
}

pub struct ObjectListNode<'a> {

    /// The actual service.
    object : Object<'a>,

    /// Next node, if any.
    next : Option<&'a ObjectListNode<'a>>,
}

impl<'a> ObjectListNode<'a> {

    pub fn new(object: Object<'a>) -> Self {
        ObjectListNode {
            object : object,
            next   : None,
        }
    }
}

impl<'a> ListNode<'a> for ObjectListNode<'a> {

    type Item = Object<'a>;

    fn elem_ref(&self) -> &Self::Item {
        &self.object
    }

    fn set_elem(&mut self, elem: Self::Item) {
        self.object = elem;
    }

    fn next_mut(&'a mut self) -> &'a mut Option<&'a Self> {
        &mut self.next
    }

    fn next_ref(&self) -> &Option<&'a Self> {
        &self.next
    }
}

impl<'a> Iterator for ServiceListNode<'a> {

    type Item = &'a ServiceListNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next
    }
}

impl<'a> Iterator for ObjectListNode<'a> {

    type Item = &'a ObjectListNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next
    }
}

/// A handle of the service in a particular object. Used to manipulate
/// with service in this object.
pub struct ServiceHandle<'a> {

    /// An object that owns this service.
    object: &'a mut Object<'a>,

    /// A list of services that contains this service.
    list: &'a mut ServiceList<'a>,

    /// Node of service list that holds this service.
    node: *mut ServiceListNode<'a>,

    /// The previous node of the list if any.
    prev_node: Option<*mut ServiceListNode<'a>>,
}

impl<'a> Handle<'a> for ServiceHandle<'a> {

    type Item = Object<'a>;

    type ListNode = ServiceListNode<'a>;

    fn item_ref(&'a self) -> &'a Self::Item {
        &self.object
    }

    fn item_mut(&'a mut self) -> &'a mut Self::Item {
        &mut self.object
    }

    fn node_ptr_mut(&'a mut self) -> &'a mut *mut Self::ListNode {
        &mut self.node
    }

    fn node_ptr(&'a self) -> *mut Self::ListNode {
        self.node
    }

    fn get_mut_prev_node_ptr(&'a mut self)
            -> &'a mut Option<*mut Self::ListNode> {
        &mut self.prev_node
    }

    fn get_prev_node_ptr(&'a self) -> Option<*mut Self::ListNode> {
        self.prev_node
    }

    fn next_node_ptr(&'a self) -> Option<*mut Self::ListNode> {
        unsafe {
            match *(*self.node_ptr()).next_ref() {
                Some(val) => Some(val as *const _ as *mut _),
                None      => None
            }
        }
    }
}

impl<'a> HandleRemovable<'a> for ServiceHandle<'a> {

    fn is_in_list_top(&'a self) -> bool {
        self.prev_node.is_none()
    }

    fn set_next_node_as_list_top(&'a self) {
        unsafe {
            let top         = &(*self.list).top as *const _ as *mut _;
            let next_node   = (*self.node_ptr()).next();

            *top = next_node;
        }
    }

    fn link_next_node_to_prev(&'a self) {
        unsafe {
            let prev = &self.prev_node as *const _ as *mut _;
            let next = (*self.node_ptr()).next();

            *prev = next;
        }
    }
}

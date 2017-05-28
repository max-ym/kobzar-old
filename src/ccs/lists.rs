use super::*;

/// List of items.
pub trait List {

    /// Node type of the list that is used to store listed items.
    type Node : ListNode;

    /// Top of the list. The first entry of it. It can be None if list
    /// is empty or a first node of the list.
    fn top(&self) -> Option<*const Self::Node>;

    /// Mutable top of the list. The first entry of it. It can be None if list
    /// is empty or a pointer to a mutable first node of the list.
    fn top_mut(&mut self) -> Option<*mut Self::Node>;

    /// Set the first node of the list. Note that each node has a pointer
    /// to next one. If you change the first node and it is pointing
    /// to another node than it was in the old node you may loose all
    /// old nodes that were connected by pointers to the old top one.
    /// If you just need to replace the first node without changing other
    /// part of the list you need another way.
    fn set_top(&mut self, top: Option<*mut Self::Node>);

    /// Append new nodes to the list top.
    fn append(&mut self, node: *mut Self::Node) {
        unsafe {
            let link_node = &mut (*node).last_node()
                    as *const _ as *mut Self::Node;

            *(*link_node).next_mut() = self.top_mut();
            self.set_top(Some(link_node));
        }
    }
}

/// The node of the list. Stores only one element of the list and a
/// pointer to the next node if any.
pub trait ListNode {

    /// The type of the item that is stored in the list.
    type Item;

    /// Reference to the item in this node.
    fn elem_ref(&self) -> &Self::Item;

    /// Reference to mutable item.
    fn elem_mut(&mut self) -> &mut Self::Item;

    /// Set the item in this node.
    fn set_elem(&mut self, top: Self::Item);

    /// Get mutable reference to the next node option.
    fn next_mut(&mut self) -> &mut Option<*mut Self>;

    /// Get a reference to the next node option.
    fn next_ref(&self) -> &Option<*mut Self>;

    /// Get the last node of the list.
    fn last_node(&self) -> &Self {
        let mut prev = self;
        loop {
            let node = *self.next_ref();
            if let Some(node) = node {
                unsafe { prev = &*node; }
            } else {
                return prev;
            }
        }
    }
}

/// Handle to edit the item of the object or service lists in any object
/// of the CCS network. It stores information about a list in which the
/// item is stored, the object that contains the list with item and other
/// data that is needed to edit the item list or item itself.
pub trait Handle : Sized {

    /// Type of the element that can be edited.
    type Item;

    /// Type of the list node that can store Item type.
    type ListNode : ListNode;

    /// Reference to the item.
    fn item_ref(&self) -> &Self::Item;

    /// Mutable reference to the item.
    fn item_mut(&mut self) -> &mut Self::Item;

    /// Mutable reference of the mutable list node that stores
    /// the item.
    fn node_ptr_mut(&mut self) -> &mut *mut Self::ListNode;

    /// Pointer to the mutable list node that stores the current item.
    fn node_ptr(&self) -> *mut Self::ListNode;

    /// Get a mutable reference to the mutable list node pointer of the
    /// previous item if any.
    fn get_mut_prev_node_ptr(&mut self) -> &mut Option<*mut Self::ListNode>;

    /// Get a pointer to the mutable list node of the previous item if any.
    fn get_prev_node_ptr(&self) -> Option<*mut Self::ListNode>;

    /// Pointer to mutable list node of the next item if any.
    fn next_node_ptr(&self) -> Option<*mut Self::ListNode>;

    /// Reference of the list node of the next item if any.
    fn next_node_ref(&self) -> Option<&Self::ListNode> {
        unsafe {
            match self.next_node_ptr() {
                Some(val) => Some(&*val),
                None      => None
            }
        }
    }
}

/// Handle that allows removing the item from the object list entirely.
trait HandleRemovable : Handle {

    /// Check if item is on the top of the list.
    fn is_on_list_top(&self) -> bool;

    /// Set next node of the list as a top of the list. All previous nodes
    /// are discarded.
    fn set_next_node_as_list_top(&self);

    /// Set the previous node to point to the next node avoiding current.
    /// This deletes current item from the list.
    fn link_next_node_to_prev(&self);

    /// Remove the item (and it's node) from the list.
    fn remove_from_list(&mut self) {
        if self.is_on_list_top() {
            self.set_next_node_as_list_top();
        } else {
            self.link_next_node_to_prev();
        }
    }
}

pub struct ServiceList {
    top : Option<*mut ServiceListNode>,
}

impl Default for ServiceList {

    fn default() -> Self {
        ServiceList { top : None }
    }
}

impl List for ServiceList {

    type Node = ServiceListNode;

    fn top(&self) -> Option<*const Self::Node> {
        match self.top {
            Some(val) => Some(val as *const _),
            None      => None
        }
    }

    fn top_mut(&mut self) -> Option<*mut Self::Node> {
        self.top
    }

    fn set_top(&mut self, top: Option<*mut Self::Node>) {
        self.top = top;
    }
}

pub struct ServiceListNode {

    /// The actual service.
    service : Service,

    /// Next node, if any.
    next : Option<*mut ServiceListNode>,
}

impl ServiceListNode {

    pub fn new(service: Service) -> Self {
        ServiceListNode {
            service : service,
            next    : None,
        }
    }
}

impl ListNode for ServiceListNode {

    type Item = Service;

    fn elem_ref(&self) -> &Self::Item {
        &self.service
    }

    fn set_elem(&mut self, elem: Self::Item) {
        self.service = elem;
    }

    fn next_mut(&mut self) -> &mut Option<*mut Self> {
        &mut self.next
    }

    fn next_ref(&self) -> &Option<*mut Self> {
        &self.next
    }

    fn elem_mut(&mut self) -> &mut Self::Item {
        &mut self.service
    }
}

pub struct ObjectList {
    top: Option<*mut ObjectListNode>,
}

impl Default for ObjectList {

    fn default() -> Self {
        ObjectList { top : None }
    }
}

impl List for ObjectList {

    type Node = ObjectListNode;

    fn top(&self) -> Option<*const Self::Node> {
        match self.top {
            Some(val) => Some(val as *const _),
            None      => None
        }
    }

    fn top_mut(&mut self) -> Option<*mut Self::Node> {
        self.top
    }

    fn set_top(&mut self, top: Option<*mut Self::Node>) {
        self.top = top;
    }
}

pub struct ObjectListNode {

    /// The actual service.
    object : Object,

    /// Next node, if any.
    next : Option<*mut ObjectListNode>,
}

impl ObjectListNode {

    pub fn new(object: Object) -> Self {
        ObjectListNode {
            object : object,
            next   : None,
        }
    }
}

impl ListNode for ObjectListNode {

    type Item = Object;

    fn elem_ref(&self) -> &Self::Item {
        &self.object
    }

    fn elem_mut(&mut self) -> &mut Self::Item {
        &mut self.object
    }

    fn set_elem(&mut self, elem: Self::Item) {
        self.object = elem;
    }

    fn next_mut(&mut self) -> &mut Option<*mut Self> {
        &mut self.next
    }

    fn next_ref(&self) -> &Option<*mut Self> {
        &self.next
    }
}

impl Iterator for ServiceListNode {

    type Item = *mut ServiceListNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.next
    }
}

impl Iterator for ObjectListNode {

    type Item = *mut ObjectListNode;

    fn next(&mut self) -> Option<Self::Item> {
        (*self).next
    }
}

/// A handle of the service in a particular object. Used to manipulate
/// with service in this object.
pub struct ServiceHandle<'a> {

    /// An object that owns this service.
    object: &'a mut Object,

    /// A list of services that contains this service.
    list: &'a mut ServiceList,

    /// Node of service list that holds this service.
    node: *mut ServiceListNode,

    /// The previous node of the list if any.
    prev_node: Option<*mut ServiceListNode>,
}

impl<'a> Handle for ServiceHandle<'a> {

    type Item = Object;

    type ListNode = ServiceListNode;

    fn item_ref(&self) -> &Self::Item {
        &self.object
    }

    fn item_mut(&mut self) -> &mut Self::Item {
        &mut self.object
    }

    fn node_ptr_mut(&mut self) -> &mut *mut Self::ListNode {
        &mut self.node
    }

    fn node_ptr(&self) -> *mut Self::ListNode {
        self.node
    }

    fn get_mut_prev_node_ptr(&mut self)
            -> &mut Option<*mut Self::ListNode> {
        &mut self.prev_node
    }

    fn get_prev_node_ptr(&self) -> Option<*mut Self::ListNode> {
        self.prev_node
    }

    fn next_node_ptr(&self) -> Option<*mut Self::ListNode> {
        unsafe {
            match *(*self.node_ptr()).next_ref() {
                Some(val) => Some(val as *const _ as *mut _),
                None      => None
            }
        }
    }
}

impl<'a> HandleRemovable for ServiceHandle<'a> {

    fn is_on_list_top(&self) -> bool {
        self.prev_node.is_none()
    }

    fn set_next_node_as_list_top(&self) {
        unsafe {
            let top         = &(*self.list).top as *const _ as *mut _;
            let next_node   = (*self.node_ptr()).next();

            *top = next_node;
        }
    }

    fn link_next_node_to_prev(&self) {
        unsafe {
            let prev = &self.prev_node as *const _ as *mut _;
            let next = (*self.node_ptr()).next();

            *prev = next;
        }
    }
}

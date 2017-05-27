use super::*;

pub trait List {

    type Item;

    fn top(&self) -> Option<Self::Item>;

    fn set_top(&mut self, top: Option<Self::Item>);
}

pub trait ListNode<'a> {

    type Item;

    fn elem_ref(&self) -> &Self::Item;

    fn set_elem(&mut self, top: Self::Item);

    fn next_mut(&'a mut self) -> &'a mut Option<&'a Self>;
}

pub trait Handle<'a> {

    type Item;

    type ListNode : ListNode<'a>;

    fn item_ref(&self) -> &Self::Item;

    fn item_mut(&'a mut self) -> &'a mut Self::Item;

    fn node_ptr_mut(&mut self) -> &mut *mut Self::ListNode;

    fn node_ptr(&self) -> *mut Self::ListNode;

    fn get_mut_prev_node_ptr(&mut self) -> &mut Option<*mut Self::ListNode>;

    fn get_prev_node_ptr(&self) -> Option<*mut Self::ListNode>;

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

    type Item = &'a ServiceListNode<'a>;

    fn top(&self) -> Option<Self::Item> {
        self.top
    }

    fn set_top(&mut self, top: Option<Self::Item>) {
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

    /// Node of service list that holds this service.
    node: *mut ServiceListNode<'a>,

    /// The previous node of the list if any.
    prev_node: Option<*mut ServiceListNode<'a>>,
}

impl<'a> Handle<'a> for ServiceHandle<'a> {

    type Item = Object<'a>;

    type ListNode = ServiceListNode<'a>;

    fn item_ref(&self) -> &Self::Item {
        &self.object
    }

    fn item_mut(&'a mut self) -> &'a mut Self::Item {
        &mut self.object
    }

    fn node_ptr_mut(&mut self) -> &mut *mut Self::ListNode {
        &mut self.node
    }

    fn node_ptr(&self) -> *mut Self::ListNode {
        self.node
    }

    fn get_mut_prev_node_ptr(&mut self) -> &mut Option<*mut Self::ListNode> {
        &mut self.prev_node
    }

    fn get_prev_node_ptr(&self) -> Option<*mut Self::ListNode> {
        self.prev_node
    }
}

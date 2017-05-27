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

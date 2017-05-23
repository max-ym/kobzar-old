#[derive(Clone, Copy)]
/// CCS Service handle.
pub struct Service {

    /// The name of the service. Used to find a service to link to.
    name : * const str,

    /// Function address in object memory to run when service is requested.
    /// The code pointer is 32-bit wide.
    func : u32,
}

/// CCS Object handle.
pub struct Object<'a> {

    /// The name of the object. Used to find a service to link to.
    name : &'a str,

    /// List of all services that this object provides to external network.
    service_list : ServiceList<'a>,

    // List of all sub-objects.
    sub_list : ObjectList<'a>,
}

struct ServiceList<'a> {
    top : Option<&'a ServiceListNode<'a>>,
}

struct ServiceListNode<'a> {

    /// The actual service.
    service : Service,

    /// Next node, if any.
    next : Option<&'a ServiceListNode<'a>>,
}

struct ObjectList<'a> {
    top: Option<&'a ObjectListNode<'a>>,
}

struct ObjectListNode<'a> {

    /// The actual service.
    object : Object<'a>,

    /// Next node, if any.
    next : Option<&'a ObjectListNode<'a>>,
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

impl<'a> ServiceHandle<'a> {

    /// A service that is handled.
    pub fn service(&self) -> &'a Service {
        unsafe { &(*self.node).service }
    }

    /// Get an object from which a service handle was created.
    pub fn into_object(self) -> &'a Object<'a> {
        self.object
    }

    /// Remove a service from the object lists.
    pub fn remove(self) -> &'a Object<'a> {
        unsafe {
            if self.prev_node.is_some() {
                let prev_node = self.prev_node.unwrap();
                // Remove this service list node from the list.
                (*prev_node).next = (*self.node).next;
            } else {
                // This node is a top of the list.
                // Move the top of the list to a next node after this one.
                self.object.service_list.top = (*self.node).next;
            }
        }

        self.into_object()
    }
}

impl<'a> Object<'a> {

    /// Find a service with a given name.
    pub fn service_with_name(&'a mut self, name: &str) ->
            Option<ServiceHandle<'a>> {
        // Create an iteration pointer.
        let i = self.service_list.top;
        let prev_i = None;

        loop {
            // List has finished. No service with given name.
            if i.is_none() {
                return None;
            }
            // Otherwise get node.
            let node = i.unwrap();

            if node.service.name == name {
                return Some(ServiceHandle {
                    object      : self,
                    node        : node as *const _ as *mut _,
                    prev_node   : prev_i
                });
            }
        }
        unreachable!();
    }

    /// Add new service to a given object. The name of the service
    /// must be unique for this object. Otherwise, service will not be
    /// added but a ServiceHandle will be returned for that service.
    /// If it succeeds to add a service then it's ServiceHandle will be
    /// returned.
    pub fn add_service(&'a mut self, service: &Service) ->
            Result<ServiceHandle<'a>, ServiceHandle<'a>> {
        unimplemented!();
    }
}

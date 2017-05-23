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

    /// A service that is handled.
    service: &'a Service,

    /// An object that owns this service.
    object: &'a Object<'a>,

    /// Node of service list that holds this service.
    node: &'a ServiceListNode<'a>,

    /// The previous node of the list if any.
    prev_node: Option<&'a ServiceListNode<'a>>,
}

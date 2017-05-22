/// CCS Service handle.
struct Service {

    /// The name of the service. Used to find a service to link to.
    name : * const str,

    /// Function address in object memory to run when service is requested.
    /// The code pointer is 32-bit wide.
    func : u32,
}

/// CCS Object handle.
struct Object<'a> {

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
    next : Option<&'a ServiceListNode<'a>>,
}

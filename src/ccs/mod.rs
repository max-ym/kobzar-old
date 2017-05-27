#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::setup;

mod lists;
use self::lists::*;

#[derive(Clone, Copy)]
/// CCS Service handle.
pub struct Service<'a> {

    /// The name of the service. Used to find a service to link to.
    name : &'a str,

    /// Function address in object memory to run when service is requested.
    /// The code pointer is 32-bit wide.
    func : u32,
}

impl<'a> Service<'a> {

    /// Create new service metadata with given fields.
    pub fn new(name: &'a str, func: u32) -> Self {
        Service {
            name : name,
            func : func,
        }
    }
}

/// CCS Object handle.
pub struct Object<'a> {

    /// The name of the object. Used to find a service to link to.
    name : &'a str,

    /// List of all services that this object provides to external network.
    pub_service_list    : ServiceList<'a>,

    // List of all sub-objects that are accessible from the external network.
    pub_obj_list        : ObjectList<'a>,

    /// List of all private services.
    priv_service_list   : ServiceList<'a>,

    /// List of all private sub-objects.
    priv_obj_list       : ObjectList<'a>,

    /// Whether parent object parent network is visible for this child.
    ///
    /// When enabled, the parent object parent network is visible
    /// only if it is visible for the parent object too. External network
    /// visibility is limited to the last object that has this option turned
    /// on.
    ///
    /// Example. There is a tree of object: Root->Foo->Bar->Baz.
    /// Baz and Bar has this option turned on. Baz can access all public
    /// services of Bar and Foo, but does not see Root object because
    /// Foo has this option turned off.
    ///
    /// This option does affect the visibility of the parent public
    /// objects and services. However, it does not affect the visibility of the
    /// private objects and services. It is affected by
    /// `is_parent_network_visible' option instead.
    is_external_network_visible : bool,

    /// Whether parent private and public object services and objects are
    /// visible for this child.
    is_parent_network_visible   : bool,
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

impl<'a> Object<'a> {

    /// Create new object with given name and empty lists.
    pub fn new(name: &'a str) -> Self {
        Object {
            name                : name,
            pub_service_list    : ServiceList::default(),
            priv_service_list   : ServiceList::default(),
            pub_obj_list        : ObjectList::default(),
            priv_obj_list       : ObjectList::default(),

            is_external_network_visible : false,
            is_parent_network_visible   : false,
        }
    }
}

mod arch;

// pub use self::arch::setup;

mod lists;
use self::lists::*;

/// Module related to channels and communication between two or multiple
/// objects or single object with itself.
mod chan;

/// Scheduler and it's related traits and structs.
mod sched;

#[derive(Clone, Copy)]
/// CCS Service handle.
pub struct Service {

    /// The name of the service. Used to find a service to link to.
    name : *const str,

    /// Function address in object memory to run when service is requested.
    func : usize,
}

impl Service {

    /// Create new service metadata with given fields.
    pub fn new(name: &str, func: usize) -> Self {
        Service {
            name : name as *const _,
            func : func,
        }
    }
}

/// CCS Object handle.
pub struct Object {

    /// The name of the object. Used to find a service to link to.
    name : *const str,

    /// List of all services that this object provides to external network.
    pub_serv_list       : ServiceList,

    // List of all sub-objects that are accessible from the external network.
    pub_obj_list        : ObjectList,

    /// List of all private services.
    priv_serv_list      : ServiceList,

    /// List of all private sub-objects.
    priv_obj_list       : ObjectList,

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

impl Object {

    /// Create new object with given name and empty lists.
    pub fn new(name: &str) -> Self {
        Object {
            name            : name as *const _,

            pub_serv_list   : Default::default(),
            priv_serv_list  : Default::default(),
            pub_obj_list    : Default::default(),
            priv_obj_list   : Default::default(),

            is_external_network_visible : false,
            is_parent_network_visible   : false,
        }
    }
}

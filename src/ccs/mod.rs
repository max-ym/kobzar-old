/// CCS Service handle.
struct Service {

    /// The name of the service. Used to find a service to link to.
    name : * const char,

    /// Function address in object memory to run when service is requested.
    /// The code pointer is 32-bit wide.
    func : u32,
}

/// CCS Object handle.
struct Object {
    // TODO
}

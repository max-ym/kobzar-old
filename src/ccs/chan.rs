use super::*;

/// Channel is used to connect two objects and perform inter-object
/// communication. This creates peer-to-peer like connection.
/// Services of objects can send and receive data through the channel.
/// They can wait for new data to come or for event that can appear in
/// connected object.
pub struct Channel<'a, 'b> {

    /// Object that requested some service.
    requester   : &'b Object,

    /// Object that responded for service request.
    responder   : &'a Object,

    /// Service that was requested. Is a service of 'responder' object.
    rsp_service : &'a Service,
}

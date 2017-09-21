use super::*;

/// Channel is used to connect processes and perform inter-process
/// communication. This creates peer-to-peer like connection.
/// Processe can send and receive data through the channel.
/// They can wait for new data to come or for event that can appear in
/// connected object.
pub trait Channel {

    /// The buffer type that is used by this channel.
    type B : Buffer;
}

/// Buffer is used by the channel to save the transfered data for each
/// process.
pub trait Buffer {

}

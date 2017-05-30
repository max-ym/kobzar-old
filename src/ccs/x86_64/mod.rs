/// Module to setup x86_64 architecture CCS.
mod setup;

/// Module for structs of register files.
mod regf;
use self::regf::*;

pub use self::setup::setup;
pub use super::Service;

/// All information needed to execute the service. Memory pages, register file
/// stack etc.
pub struct ServiceData {

    /// General information about service.
    base    : Service,
}

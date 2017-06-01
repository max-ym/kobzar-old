/// Module to setup x86_64 architecture CCS.
mod setup;

/// Module for structs of register files.
mod regf;
use self::regf::*;

pub use self::setup::setup;
pub use super::Service;

use ::mem::paging::MainPageMap;

/// All information needed to execute the service. Memory pages, register file
/// stack etc.
pub struct ServiceData {

    /// General information about service.
    base    : Service,

    reg_gp  : Option<*mut GpRegisterFile>,
    reg_fpr : Option<*mut FprRegisterFile>,
    reg_mmx : Option<*mut MmxRegisterFile>,
    reg_xmm : Option<*mut XmmRegisterFile>,
    reg_ymm : Option<*mut YmmRegisterFile>,

    /// Page table of this service.
    page_table  : MainPageMap,

    // TODO
}

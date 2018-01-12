/// The type of process counter. Must be unsigned int enough big to
/// index all processes in the system.
///
/// TODO connect to appropriate module.
type ProcessCount = u32;

/// Structures related to 2MiB and 4KiB pages.
mod page;
use self::page::*;

/// 2M page stack.
mod stack;

/// 2M page tree and related stuff.
mod tree;

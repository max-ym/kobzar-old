// TODO: mod needs revision.

mod page;
pub use self::page::*;

use super::tentr::*;

/// Main page map.
pub type MainPageMap = P4;

#[repr(packed)]
pub struct P4 {
    entries : [P4E; 512]
}

#[repr(packed)]
pub struct P3 {
    entries : [P3E; 512]
}

#[repr(packed)]
pub struct P2 {
    entries : [P2E; 512]
}

#[repr(packed)]
pub struct P1 {
    entries : [P1E; 512]
}

impl Default for P1 {

    fn default() -> Self {
        P1 {
            entries : [Default::default(); 512]
        }
    }
}

impl Default for P2 {

    fn default() -> Self {
        P2 {
            entries : [Default::default(); 512]
        }
    }
}

impl Default for P3 {

    fn default() -> Self {
        P3 {
            entries : [Default::default(); 512]
        }
    }
}

impl Default for P4 {

    fn default() -> Self {
        P4 {
            entries : [Default::default(); 512]
        }
    }
}

macro_rules! impl_table {
    ($x:ident, $e:ident) => (
        impl Table for $x {

            type EntryType = $e;

            unsafe fn entry_ref<'a, 'b>(&'a self, index: u16)
                    -> &'b Self::EntryType {
                &*(&self.entries[index as usize] as *const _)
            }

            unsafe fn entry_mut<'a, 'b>(&'a self, index: u16)
                    -> &'b mut Self::EntryType {
                &mut *(&self.entries[index as usize] as *const _ as *mut _)
            }

            fn limit(&self) -> u16 {
                self.entries.len() as _ // always 512
            }

            fn limit_broken_by(&self, index: u16) -> bool {
                index >= self.limit()
            }
        }
    );
}

impl_table!(P1, P1E);
impl_table!(P2, P2E);
impl_table!(P3, P3E);
impl_table!(P4, P4E);

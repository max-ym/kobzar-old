/// Entry from EntryTable.
pub trait Entry : Sized {

    /// Get size in bytes of an entry type.
    fn size() -> usize {
        ::core::mem::size_of::<Self>()
    }
}

/// The handle of an entry.
pub trait EntryHandle {
}

/// Table of entries.
pub trait Table {

    type Handle : EntryHandle;

    /// Get entry reference by it's index in the entry table.
    /// Does not check if entry is actually present in the table.
    unsafe fn entry_handle<'a, 'b>(&'a self, index: u16)
            -> &'b Self::Handle;

    /// Get entry reference by it's index in the entry table.
    /// Return None if entry is not present.
    fn get_entry_handle<'a, 'b>(&'a self, index: u16)
            -> Option<&'b Self::Handle> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.entry_ref(index) })
        }
    }

    /// Get limit of entry table.
    fn limit(&self) -> u16;

    /// Check if given index breaks the limit of entry table.
    /// If so, there is no entry with given index in the table.
    fn limit_broken_by(&self, index: u16) -> bool;

    /// Get address of the table.
    fn addr(&self) -> u64;
}

/// Specific interpretation of some general entries in table. For
/// example, IDT has gates which are entries of this table. But
/// those entries can be interrupt gates or trap gates. So each
/// of the gates implement this trait to allow reinterpreting them as
/// more specific entries to access additional functions and properties.
pub trait EntryVariant<E: Entry> : Entry {

    /// Try to get reference to an entry variant. If it cannot be
    /// interpreted in a requested way, None will be returned.
    fn try_variant_ref(&self) -> Option<&E>;

    /// Try to get mutable reference to an entry variant. If it cannot be
    /// interpreted in a requested way, None will be returned.
    fn try_variant_mut(&mut self) -> Option<&mut E>;
}

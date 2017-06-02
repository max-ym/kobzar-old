/// Entry from EntryTable.
pub trait Entry : Sized {

    /// Get size in bytes of an entry type.
    fn size() -> usize {
        ::core::mem::size_of::<Self>()
    }
}

/// Table of entries.
pub trait Table {

    type EntryType : Entry;

    /// Get entry reference by it's index in the entry table.
    /// Does not check if entry is actually present in the table.
    unsafe fn entry_ref<'a, 'b>(&'a self, index: u16)
            -> &'b Self::EntryType;

    /// Get entry reference by it's index in the entry table.
    /// Return None if entry is not present.
    fn get_entry_ref<'a, 'b>(&'a self, index: u16)
            -> Option<&'b Self::EntryType> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.entry_ref(index) })
        }
    }

    /// Get mutable reference to entry in table by it's index. Does
    /// not check if entry is actually present in the table.
    unsafe fn descriptor_mut<'a, 'b>(&'a self, index: u16)
            -> &'b mut Self::EntryType;

    /// Get mutable reference to entry in table by it's index.
    /// If descriptor is abscent the None is returned.
    fn get_entry_mut<'a, 'b>(&'a self, index: u16)
            -> Option<&'b mut Self::EntryType> {
        if self.limit_broken_by(index) {
            None
        } else {
            Some(unsafe { self.descriptor_mut(index) })
        }
    }

    /// Get limit of entry table.
    fn limit(&self) -> u16;

    /// Check if given index breaks the limit of entry table.
    /// If so, there is no entry with given index in the table.
    fn limit_broken_by(&self, index: u16) -> bool {
        self.limit() < index * Self::EntryType::size() as u16 + 1
    }
}

/// Specific interpretation of some general entries in table. For
/// example, IDT has gates which are entreis of this table. But
/// those entries can be interrupt gates or trap gates. So each
/// of the gates implement this trait to allow reinterpreting them as
/// more specific entries to access additional functions and properties.
pub trait EntryVariant<E: Entry> : Entry {

    /// Try to get reference to an entry variant. If it cannot be
    /// interpreted in a requested way, None will be returned.
    fn try_variant_ref(value: &E) -> Option<&Self>;

    /// Try to get mutable reference to an entry variant. If it cannot be
    /// interpreted in a requested way, None will be returned.
    fn try_variant_mut(value: &mut E) -> Option<&mut Self>;
}

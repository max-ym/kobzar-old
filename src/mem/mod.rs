mod arch;
pub use self::arch::*;

/// Simple wrapper for memory address.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Address {
    addr    : usize,
}

impl Into<usize> for Address {

    fn into(self) -> usize {
        self.addr
    }
}

impl From<usize> for Address {

    fn from(addr: usize) -> Self {
        Address { addr:addr }
    }
}

impl Into<isize> for Address {

    fn into(self) -> isize {
        self.addr as _
    }
}

impl From<isize> for Address {

    fn from(addr: isize) -> Self {
        Address { addr:addr as _ }
    }
}

impl Address {

    /// Convert this address to a pointer of a given type.
    pub fn as_ptr<T>(&self) -> *const T {
        self.addr as _
    }

    /// Convert this address to a mutable pointer of a given type.
    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.as_ptr::<T>() as _
    }

    /// Get reference to the value.
    ///
    /// # Safety
    /// Caller must ensure that this address points to a valid value.
    pub unsafe fn as_ref<T>(&self) -> &T {
        &*self.as_ptr::<T>()
    }

    /// Get mutable reference to the value.
    ///
    /// # Safety
    /// Caller must ensure that this address points to a valid value.
    pub unsafe fn as_ref_mut<T>(&self) -> &mut T {
        &mut *self.as_mut_ptr()
    }

    /// Get the address of a given value reference.
    pub fn address_of<T>(t: &T) -> Self {
        (t as *const T as usize).into()
    }
}

/// Minimal memory allocator.
pub trait Allocator {

    /// Allocate next range of bytes.
    fn alloc(&mut self, size: usize) -> Address;
}

/// Allocator for particular data type.
pub trait TypedAllocator : Allocator {

    /// The type of a value to be allocated.
    type T;

    /// Allocate array of value type (uninitialized).
    fn next(&mut self, count: usize) -> *mut Self::T {
        use ::core::mem::size_of;

        let addr: usize = self.alloc(size_of::<Self::T>() * count).into();
        addr as *const Self::T as _
    }
}

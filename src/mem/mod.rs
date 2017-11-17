mod arch;
pub use self::arch::*;

use Address;

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
